// Copyright (c) MySocial, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module social_contracts::upgrade_tests {
    use social_contracts::upgrade;

    #[test]
    /// Greenfield genesis ships at package version 0.
    fun test_current_version_is_genesis() {
        assert!(upgrade::current_version() == 0, 0);
    }

    #[test]
    /// Test utility function for creating properly-sized digests
    fun test_create_test_digest() {
        // Test with shorter than 32 bytes
        let input_short = b"too short";
        let result_short = upgrade::create_test_digest(input_short);
        assert!(std::vector::length(&result_short) == 32, 0);
        
        // Test with exactly 32 bytes
        let input_exact = create_test_digest();
        let result_exact = upgrade::create_test_digest(input_exact);
        assert!(std::vector::length(&result_exact) == 32, 0);
        
        // Test with longer than 32 bytes
        let mut input_long = std::vector::empty<u8>();
        let mut i = 0u64;
        while (i < 64u64) {
            std::vector::push_back(&mut input_long, (i as u8));
            i = i + 1u64;
        };
        let result_long = upgrade::create_test_digest(input_long);
        assert!(std::vector::length(&result_long) == 32, 0);
    }

    #[test]
    /// Production migration order (after a package upgrade, using `UpgradeAdminCap`):
    /// 1. Shared configs: platform, profile, post, mydata, subscription, memory,
    ///    social_proof_tokens, spot, insurance, ai_credit, and related fee/config objects.
    /// 2. Shared registries: social_graph, username, mydata, governance, token, spot claim, etc.
    /// 3. Per-object migrations: profiles, posts, vaults, balances, policies, markets, etc.
    /// Operator entry points are listed in `tests/interact.sh` upgrade_menu.
    fun test_documented_migration_order_placeholder() {
        assert!(upgrade::test_pre_upgrade_object_version() == 0, 0);
    }
    
    /// Create a test 32-byte digest
    fun create_test_digest(): vector<u8> {
        let mut result = std::vector::empty<u8>();
        let mut i = 0u64;
        while (i < 32u64) {
            std::vector::push_back(&mut result, (i as u8));
            i = i + 1u64;
        };
        result
    }
}
