//! In-process orderbook indexer for tools like `myso start` that already run the network binary.
//!
//! This connects to Postgres, runs migrations, consumes checkpoints (e.g. from the fullnode data
//! ingestion dir), and registers pipeline handlers. It does **not** publish or deploy packages;
//! system package addresses come from genesis (`myso_types` built-ins, same as validators).

use crate::handlers::balances_handler::BalancesHandler;
use crate::handlers::conditional_order_added_handler::ConditionalOrderAddedHandler;
use crate::handlers::conditional_order_cancelled_handler::ConditionalOrderCancelledHandler;
use crate::handlers::conditional_order_executed_handler::ConditionalOrderExecutedHandler;
use crate::handlers::conditional_order_insufficient_funds_handler::ConditionalOrderInsufficientFundsHandler;
use crate::handlers::deposit_collateral_handler::DepositCollateralHandler;
use crate::handlers::flash_loan_handler::FlashLoanHandler;
use crate::handlers::liquidation_handler::LiquidationHandler;
use crate::handlers::loan_borrowed_handler::LoanBorrowedHandler;
use crate::handlers::loan_repaid_handler::LoanRepaidHandler;
use crate::handlers::maintainer_cap_updated_handler::MaintainerCapUpdatedHandler;
use crate::handlers::maintainer_fees_withdrawn_handler::MaintainerFeesWithdrawnHandler;
use crate::handlers::margin_manager_created_handler::MarginManagerCreatedHandler;
use crate::handlers::margin_pool_config_updated_handler::MarginPoolConfigUpdatedHandler;
use crate::handlers::margin_pool_created_handler::MarginPoolCreatedHandler;
use crate::handlers::myso_burned_handler::MysoBurnedHandler;
use crate::handlers::order_fill_handler::OrderFillHandler;
use crate::handlers::order_update_handler::OrderUpdateHandler;
use crate::handlers::orderbook_pool_config_updated_handler::OrderbookPoolConfigUpdatedHandler;
use crate::handlers::orderbook_pool_registered_handler::OrderbookPoolRegisteredHandler;
use crate::handlers::orderbook_pool_updated_handler::OrderbookPoolUpdatedHandler;
use crate::handlers::orderbook_pool_updated_registry_handler::OrderbookPoolUpdatedRegistryHandler;
use crate::handlers::pause_cap_updated_handler::PauseCapUpdatedHandler;
use crate::handlers::pool_created_handler::PoolCreatedHandler;
use crate::handlers::pool_price_handler::PoolPriceHandler;
use crate::handlers::proposals_handler::ProposalsHandler;
use crate::handlers::protocol_fees_increased_handler::ProtocolFeesIncreasedHandler;
use crate::handlers::protocol_fees_withdrawn_handler::ProtocolFeesWithdrawnHandler;
use crate::handlers::rebates_handler::RebatesHandler;
use crate::handlers::referral_fee_event_handler::ReferralFeeEventHandler;
use crate::handlers::referral_fees_claimed_handler::ReferralFeesClaimedHandler;
use crate::handlers::stakes_handler::StakesHandler;
use crate::handlers::supplier_cap_minted_handler::SupplierCapMintedHandler;
use crate::handlers::supply_referral_minted_handler::SupplyReferralMintedHandler;
use crate::handlers::trade_params_update_handler::TradeParamsUpdateHandler;
use crate::handlers::vote_handler::VotesHandler;
use crate::handlers::withdraw_collateral_handler::WithdrawCollateralHandler;
use crate::handlers::{
    asset_supplied_handler::AssetSuppliedHandler, asset_withdrawn_handler::AssetWithdrawnHandler,
    interest_params_updated_handler::InterestParamsUpdatedHandler,
};
use crate::{OrderbookEnv, Package};
use anyhow::Context;
use myso_futures::service::Service;
use myso_indexer_alt_framework::ingestion::ClientArgs;
use myso_indexer_alt_framework::ingestion::IngestionConfig;
use myso_indexer_alt_framework::{Indexer, IndexerArgs};
use myso_indexer_alt_metrics::db::DbConnectionStatsCollector;
use myso_indexer_alt_metrics::{MetricsArgs, MetricsService};
use myso_indexer_alt_orderbook_schema::MIGRATIONS;
use myso_pg_db::{Db, DbArgs};
use prometheus::Registry;
use url::Url;

/// Postgres + checkpoints + handlers, returned as a [`Service`] to merge with `myso start`'s RPC supervisor.
pub async fn build_orderbook_indexer(
    database_url: Url,
    db_args: DbArgs,
    indexer_args: IndexerArgs,
    client_args: ClientArgs,
    metrics_args: MetricsArgs,
    registry: &Registry,
    env: OrderbookEnv,
    packages: &[Package],
) -> anyhow::Result<Service> {
    let store = Db::for_write(database_url, db_args)
        .await
        .context("Failed to connect to orderbook database")?;

    store
        .run_migrations(Some(&MIGRATIONS))
        .await
        .context("Failed to run orderbook migrations")?;

    registry.register(Box::new(DbConnectionStatsCollector::new(
        Some("orderbook_indexer_db"),
        store.clone(),
    )))?;

    let metrics = MetricsService::new(metrics_args, registry.clone());

    let mut indexer = Indexer::new(
        store,
        indexer_args,
        client_args,
        IngestionConfig::default(),
        Some("orderbook"),
        metrics.registry(),
    )
    .await
    .context("Failed to create orderbook indexer")?;

    register_orderbook_packages(&mut indexer, env, packages).await?;

    let s_indexer = indexer
        .run()
        .await
        .context("Failed to start orderbook indexer")?;
    let s_metrics = metrics.run().await?;

    Ok(s_indexer.attach(s_metrics))
}

async fn register_orderbook_packages(
    indexer: &mut Indexer<Db>,
    env: OrderbookEnv,
    packages: &[Package],
) -> anyhow::Result<()> {
    for package in packages {
        match package {
            Package::Orderbook => {
                indexer
                    .concurrent_pipeline(BalancesHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(PoolCreatedHandler::new(env), Default::default())
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
                indexer
                    .concurrent_pipeline(DepositCollateralHandler::new(env), Default::default())
                    .await?;
                indexer
                    .concurrent_pipeline(WithdrawCollateralHandler::new(env), Default::default())
                    .await?;
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
    Ok(())
}
