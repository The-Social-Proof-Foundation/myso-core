use alloy_multiprovider_strategy::{
    Address, HealthStatus, MultiProvider, MultiProviderConfig, Provider,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = MultiProviderConfig::new(
        vec![
            "https://eth.llamarpc.com".to_string(),
            "https://rpc.ankr.com/eth".to_string(),
            "https://ethereum.publicnode.com".to_string(),
            "https://1rpc.io/eth".to_string(),
        ],
        2,
    )
    .with_health_check_interval(Duration::from_secs(300))
    .with_max_block_delta(5)
    .with_max_healthy_latency(Duration::from_secs(5))
    .with_request_timeout(Duration::from_secs(30));

    println!("Connecting to {} RPC providers...", config.rpc_urls.len());

    let provider = MultiProvider::new(config).await?;

    println!(
        "Connected! {} providers healthy out of {} total",
        provider.healthy_provider_count(),
        provider.provider_count()
    );

    println!("\nProvider Health Status:");
    println!("{:-<60}", "");
    for (url, status) in provider.get_provider_health() {
        let status_str = match status {
            HealthStatus::Healthy => "✓ Healthy",
            HealthStatus::HighLatency => "⚠ High Latency",
            HealthStatus::Behind => "⚠ Behind",
            HealthStatus::Unreachable => "✗ Unreachable",
            HealthStatus::Unknown => "? Unknown",
        };
        println!("  {} - {}", status_str, url);
    }
    println!("{:-<60}", "");

    let block_number = provider.get_block_number().await?;
    println!("Current block number: {}", block_number);

    let chain_id = provider.get_chain_id().await?;
    println!("Chain ID: {}", chain_id);

    let gas_price = provider.get_gas_price().await?;
    println!("Gas price: {} gwei", gas_price / 1_000_000_000);

    // vitalik.eth
    let address: Address = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".parse()?;
    println!("\nFetching balance of {}...", address);
    let balance = provider.get_balance(address).await?;
    println!(
        "Balance: {} ETH",
        balance.to_string().parse::<f64>().unwrap_or(0.0) / 1e18
    );

    provider.shutdown();
    println!("\nShutdown complete!");

    Ok(())
}
