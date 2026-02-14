use anyhow::Context;
use clap::Parser;
use orderbook_indexer::handlers::balances_handler::BalancesHandler;
use orderbook_indexer::handlers::myso_burned_handler::MysoBurnedHandler;
use orderbook_indexer::handlers::flash_loan_handler::FlashLoanHandler;
use orderbook_indexer::handlers::order_fill_handler::OrderFillHandler;
use orderbook_indexer::handlers::order_update_handler::OrderUpdateHandler;
use orderbook_indexer::handlers::pool_price_handler::PoolPriceHandler;
use orderbook_indexer::handlers::proposals_handler::ProposalsHandler;
use orderbook_indexer::handlers::rebates_handler::RebatesHandler;
use orderbook_indexer::handlers::referral_fee_event_handler::ReferralFeeEventHandler;
use orderbook_indexer::handlers::stakes_handler::StakesHandler;
use orderbook_indexer::handlers::trade_params_update_handler::TradeParamsUpdateHandler;
use orderbook_indexer::handlers::vote_handler::VotesHandler;

// Margin Manager Events
use orderbook_indexer::handlers::liquidation_handler::LiquidationHandler;
use orderbook_indexer::handlers::loan_borrowed_handler::LoanBorrowedHandler;
use orderbook_indexer::handlers::loan_repaid_handler::LoanRepaidHandler;
use orderbook_indexer::handlers::margin_manager_created_handler::MarginManagerCreatedHandler;

// Margin Pool Operations Events
use orderbook_indexer::handlers::asset_supplied_handler::AssetSuppliedHandler;
use orderbook_indexer::handlers::asset_withdrawn_handler::AssetWithdrawnHandler;
use orderbook_indexer::handlers::maintainer_fees_withdrawn_handler::MaintainerFeesWithdrawnHandler;
use orderbook_indexer::handlers::protocol_fees_withdrawn_handler::ProtocolFeesWithdrawnHandler;
use orderbook_indexer::handlers::supplier_cap_minted_handler::SupplierCapMintedHandler;
use orderbook_indexer::handlers::supply_referral_minted_handler::SupplyReferralMintedHandler;

// Margin Pool Admin Events
use orderbook_indexer::handlers::orderbook_pool_updated_handler::OrderbookPoolUpdatedHandler;
use orderbook_indexer::handlers::interest_params_updated_handler::InterestParamsUpdatedHandler;
use orderbook_indexer::handlers::margin_pool_config_updated_handler::MarginPoolConfigUpdatedHandler;
use orderbook_indexer::handlers::margin_pool_created_handler::MarginPoolCreatedHandler;

// Margin Registry Events
use orderbook_indexer::handlers::orderbook_pool_config_updated_handler::OrderbookPoolConfigUpdatedHandler;
use orderbook_indexer::handlers::orderbook_pool_registered_handler::OrderbookPoolRegisteredHandler;
use orderbook_indexer::handlers::orderbook_pool_updated_registry_handler::OrderbookPoolUpdatedRegistryHandler;
use orderbook_indexer::handlers::maintainer_cap_updated_handler::MaintainerCapUpdatedHandler;
use orderbook_indexer::handlers::pause_cap_updated_handler::PauseCapUpdatedHandler;

// Protocol Fees Events
use orderbook_indexer::handlers::protocol_fees_increased_handler::ProtocolFeesIncreasedHandler;
use orderbook_indexer::handlers::referral_fees_claimed_handler::ReferralFeesClaimedHandler;

// Collateral Events
use orderbook_indexer::handlers::deposit_collateral_handler::DepositCollateralHandler;
use orderbook_indexer::handlers::withdraw_collateral_handler::WithdrawCollateralHandler;

// TPSL (Take Profit / Stop Loss) Events
use orderbook_indexer::handlers::conditional_order_added_handler::ConditionalOrderAddedHandler;
use orderbook_indexer::handlers::conditional_order_cancelled_handler::ConditionalOrderCancelledHandler;
use orderbook_indexer::handlers::conditional_order_executed_handler::ConditionalOrderExecutedHandler;
use orderbook_indexer::handlers::conditional_order_insufficient_funds_handler::ConditionalOrderInsufficientFundsHandler;

use orderbook_indexer::OrderbookEnv;
use myso_indexer_alt_orderbook_schema::MIGRATIONS;
use prometheus::Registry;
use std::net::SocketAddr;
use myso_indexer_alt_framework::ingestion::ingestion_client::IngestionClientArgs;
use myso_indexer_alt_framework::ingestion::streaming_client::StreamingClientArgs;
use myso_indexer_alt_framework::ingestion::{ClientArgs, IngestionConfig};
use myso_indexer_alt_framework::{Indexer, IndexerArgs};
use myso_indexer_alt_metrics::db::DbConnectionStatsCollector;
use myso_indexer_alt_metrics::{MetricsArgs, MetricsService};
use myso_pg_db::{Db, DbArgs};

use url::Url;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Package {
    /// Index Orderbook core events (order fills, updates, pools, etc.)
    Orderbook,
    /// Index Orderbook margin events (lending, borrowing, liquidations, etc.)
    OrderbookMargin,
}

#[derive(Parser)]
#[clap(rename_all = "kebab-case", author, version)]
struct Args {
    #[command(flatten)]
    db_args: DbArgs,
    #[command(flatten)]
    indexer_args: IndexerArgs,
    #[command(flatten)]
    streaming_args: StreamingClientArgs,
    #[clap(env, long, default_value = "0.0.0.0:9184")]
    metrics_address: SocketAddr,
    #[clap(
        env,
        long,
        default_value = "postgres://postgres:postgrespw@localhost:5432/orderbook"
    )]
    database_url: Url,
    /// Orderbook environment, defaulted to MYSO mainnet.
    #[clap(env, long)]
    env: OrderbookEnv,
    /// Packages to index events for (can specify multiple)
    #[clap(long, value_enum, default_values = ["orderbook", "orderbook-margin"])]
    packages: Vec<Package>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    let Args {
        db_args,
        indexer_args,
        streaming_args,
        metrics_address,
        database_url,
        env,
        packages,
    } = Args::parse();

    let registry = Registry::new_custom(Some("orderbook".into()), None)
        .context("Failed to create Prometheus registry.")?;
    let metrics = MetricsService::new(MetricsArgs { metrics_address }, registry.clone());

    // Prepare the store for the indexer
    let store = Db::for_write(database_url, db_args)
        .await
        .context("Failed to connect to database")?;

    store
        .run_migrations(Some(&MIGRATIONS))
        .await
        .context("Failed to run pending migrations")?;

    registry.register(Box::new(DbConnectionStatsCollector::new(
        Some("orderbook_indexer_db"),
        store.clone(),
    )))?;

    let mut indexer = Indexer::new(
        store,
        indexer_args,
        ClientArgs {
            ingestion: IngestionClientArgs {
                remote_store_url: Some(env.remote_store_url()),
                ..Default::default()
            },
            streaming: streaming_args,
        },
        IngestionConfig::default(),
        None,
        metrics.registry(),
    )
    .await?;

    // Register handlers based on selected packages
    for package in &packages {
        match package {
            Package::Orderbook => {
                // Orderbook core event handlers
                indexer
                    .concurrent_pipeline(BalancesHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(MysoBurnedHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(FlashLoanHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(OrderFillHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(OrderUpdateHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(PoolPriceHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(ProposalsHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(RebatesHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(ReferralFeeEventHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(StakesHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(TradeParamsUpdateHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(VotesHandler::new(env), Default::default())
                    .await?;
            }
            Package::OrderbookMargin => {
                indexer
                    .concurrent_pipeline(MarginManagerCreatedHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(LoanBorrowedHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(LoanRepaidHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(LiquidationHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(AssetSuppliedHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(AssetWithdrawnHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(MarginPoolCreatedHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(OrderbookPoolUpdatedHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(InterestParamsUpdatedHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(
                        MarginPoolConfigUpdatedHandler::new(env),
                        Default::default(),
                    )
                    .await?;
                indexer
                    .concurrent_pipeline(MaintainerCapUpdatedHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(
                        OrderbookPoolRegisteredHandler::new(env),
                        Default::default(),
                    )
                    .await?;
                indexer
                    .concurrent_pipeline(
                        OrderbookPoolUpdatedRegistryHandler::new(env),
                        Default::default(),
                    )
                    .await?;
                indexer
                    .concurrent_pipeline(
                        OrderbookPoolConfigUpdatedHandler::new(env),
                        Default::default(),
                    )
                    .await?;
                indexer
                    .concurrent_pipeline(
                        MaintainerFeesWithdrawnHandler::new(env),
                        Default::default(),
                    )
                    .await?;
                indexer
                    .concurrent_pipeline(ProtocolFeesWithdrawnHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(SupplierCapMintedHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(SupplyReferralMintedHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(PauseCapUpdatedHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(ProtocolFeesIncreasedHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(ReferralFeesClaimedHandler::new(env), Default::default())
                    .await?;

                // Collateral Events
                indexer
                    .concurrent_pipeline(DepositCollateralHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(WithdrawCollateralHandler::new(env), Default::default())
                    .await?;

                // TPSL (Take Profit / Stop Loss) Events
                indexer
                    .concurrent_pipeline(ConditionalOrderAddedHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(
                        ConditionalOrderCancelledHandler::new(env),
                        Default::default(),
                    )
                    .await?;
                indexer
                    .concurrent_pipeline(
                        ConditionalOrderExecutedHandler::new(env),
                        Default::default(),
                    )
                    .await?;
                indexer
                    .concurrent_pipeline(
                        ConditionalOrderInsufficientFundsHandler::new(env),
                        Default::default(),
                    )
                    .await?;
            }
        }
    }

    let s_indexer = indexer.run().await?;
    let s_metrics = metrics.run().await?;

    s_indexer.attach(s_metrics).main().await?;
    Ok(())
}
