// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Genesis bootstrap for MySocial — initializes shared objects and distributes admin caps once.

#[allow(duplicate_alias, unused_use, lint(public_entry))]
module social_contracts::bootstrap {
    use myso::{
        tx_context::{Self, TxContext},
        clock::{Self, Clock},
        transfer,
        bootstrap_key::{Self, BootstrapKey}
    };

    use social_contracts::upgrade::{Self, UpgradeAdminCap};
    use social_contracts::social_proof_tokens::{Self, SocialProofTokensAdminCap};
    use social_contracts::post::{Self, PostAdminCap};
    use social_contracts::proof_of_creativity::{Self, PoCAdminCap};
    use social_contracts::platform::{Self, PlatformAdminCap};
    use social_contracts::governance::{Self, GovernanceAdminCap};
    use social_contracts::mydata::{Self, MyDataAdminCap, MyDataPoolAdminCap};
    use social_contracts::social_proof_of_truth::{Self, SpotAdminCap, SpotOracleAdminCap};
    use social_contracts::insurance::{Self, InsuranceAdminCap};
    use social_contracts::profile::{Self, EcosystemTreasuryAdminCap, EcosystemBadgeAdminCap};

    use myso::coin::{Self, CoinCreationAdminCap};
    use myso::package::{Self, PackagePublishingAdminCap};
    use orderbook::registry::{Self as ob_registry};

    const ENotSystemAddress: u64 = 0;
    const ENotGenesis: u64 = 1;

    /// Initialize MySocial shared objects, mint admin caps for `admin`, and seal the bootstrap key.
    public(package) fun init_social_platform(
        registry: &mut ob_registry::Registry,
        bootstrap_key: &mut BootstrapKey,
        clock: &Clock,
        admin: address,
        ctx: &mut TxContext,
    ) {
        bootstrap_key::assert_not_used(bootstrap_key);

        social_contracts::platform::bootstrap_init(ctx);
        social_contracts::social_graph::bootstrap_init(ctx);
        social_contracts::profile::bootstrap_init(ctx);
        social_contracts::block_list::bootstrap_init(ctx);
        social_contracts::mydata::bootstrap_init(ctx);
        social_contracts::memory::bootstrap_init(ctx);
        let spot_governance_registry_id =
            social_contracts::governance::bootstrap_init(clock, admin, ctx);
        social_contracts::post::bootstrap_init(ctx);
        social_contracts::social_proof_tokens::bootstrap_init(ctx);
        social_contracts::proof_of_creativity::bootstrap_init(ctx);
        social_contracts::social_proof_of_truth::bootstrap_init(
            clock,
            spot_governance_registry_id,
            ctx,
        );
        social_contracts::insurance::bootstrap_init(ctx);

        transfer::public_transfer(upgrade::create_upgrade_admin_cap(ctx), admin);
        transfer::public_transfer(
            social_proof_tokens::create_social_proof_tokens_admin_cap(ctx),
            admin,
        );
        transfer::public_transfer(post::create_post_admin_cap(ctx), admin);
        transfer::public_transfer(proof_of_creativity::create_poc_admin_cap(ctx), admin);
        transfer::public_transfer(platform::create_platform_admin_cap(ctx), admin);
        transfer::public_transfer(governance::create_governance_admin_cap(ctx), admin);
        transfer::public_transfer(mydata::create_mydata_admin_cap(ctx), admin);
        transfer::public_transfer(mydata::create_mydata_pool_admin_cap(ctx), admin);
        transfer::public_transfer(social_proof_of_truth::create_spot_admin_cap(ctx), admin);
        transfer::public_transfer(
            social_proof_of_truth::create_spot_oracle_admin_cap(ctx),
            admin,
        );
        transfer::public_transfer(profile::create_ecosystem_treasury_admin_cap(ctx), admin);
        transfer::public_transfer(profile::create_ecosystem_badge_admin_cap(ctx), admin);
        transfer::public_transfer(insurance::create_insurance_admin_cap(ctx), admin);
        transfer::public_transfer(
            coin::create_coin_creation_admin_cap_for_bootstrap(bootstrap_key, ctx),
            admin,
        );
        transfer::public_transfer(
            package::create_package_publishing_admin_cap_for_bootstrap(bootstrap_key, ctx),
            admin,
        );

        let orderbook_admin_cap =
            ob_registry::create_orderbook_admin_cap_for_bootstrap(bootstrap_key, ctx);
        ob_registry::set_treasury_address(registry, admin, &orderbook_admin_cap);
        transfer::public_transfer(orderbook_admin_cap, admin);

        bootstrap_key::finalize_bootstrap(bootstrap_key);
    }

    #[allow(unused_function)]
    /// Called exactly once during genesis by the genesis builder.
    fun init_at_genesis(
        registry: &mut ob_registry::Registry,
        bootstrap_key: &mut BootstrapKey,
        clock: &Clock,
        admin: address,
        ctx: &mut TxContext,
    ) {
        assert!(ctx.sender() == @0x0, ENotSystemAddress);
        assert!(ctx.epoch() == 0, ENotGenesis);
        init_social_platform(registry, bootstrap_key, clock, admin, ctx);
    }
}
