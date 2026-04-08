# alloy-multiprovider-strategy

A multi-provider wrapper for [alloy](https://alloy.rs/) with the following features:

- **Multiple RPC Providers**: Connect to multiple Ethereum RPC endpoints simultaneously
- **Quorum Consensus**: Require a configurable number of providers to agree on responses
- **Automatic Health Ranking**: Providers are ranked by health (block height and latency)
- **Periodic Health Checks**: Background task monitors provider health at configurable intervals
- **Graceful Fallback**: If top providers fail, automatically try the next ranked providers
- **Full Provider Trait**: Implements the complete `alloy::Provider` trait - no need for special methods!

## Architecture

This library is using the Alloy wrapper abstractions (https://alloy.rs/guides/rpc-provider-abstraction/) and so is effectively just a wrapper on top of the alloy_provider with the exact same interface / methods.

So we can just swap the provider library to use it without any code change.
