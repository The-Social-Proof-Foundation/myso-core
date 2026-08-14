// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, duplicate_alias)]
module social_contracts::derivative_graph_tests {
    use social_contracts::derivative_graph::{Self as dg};
    use myso::object::{Self, ID};
    use myso::address;

    const E_SELF_EDGE: u64 = 1;
    const E_CYCLE_DETECTED: u64 = 2;
    const E_ANCESTOR_LIMIT: u64 = 3;

    fun id_a(): ID { object::id_from_address(@0xA1) }
    fun id_b(): ID { object::id_from_address(@0xB1) }
    fun id_c(): ID { object::id_from_address(@0xC1) }

    #[test]
    fun test_empty_ancestry_on_root() {
        let ancestry = dg::test_empty_ancestry();
        assert!(vector::length(dg::ancestor_ids(&ancestry)) == 0);
        assert!(dg::ancestry_version(&ancestry) == 0);
    }

    #[test]
    fun test_canonical_union_dedupes_and_sorts() {
        let a = vector[id_a(), id_c()];
        let b = vector[id_b(), id_a()];
        let merged = dg::test_canonical_union(&a, &b, option::none());
        assert!(vector::length(&merged) == 3);
        assert!(*vector::borrow(&merged, 0) == id_a());
        assert!(*vector::borrow(&merged, 1) == id_b());
        assert!(*vector::borrow(&merged, 2) == id_c());
    }

    #[test]
    fun test_merge_ancestry_builds_chain() {
        let parent_id = id_a();
        let mut child = dg::test_empty_ancestry();
        dg::test_merge_ancestry_for_edge(&mut child, parent_id, &vector[]);
        assert!(vector::length(dg::ancestor_ids(&child)) == 1);
        assert!(*vector::borrow(dg::ancestor_ids(&child), 0) == parent_id);
        assert!(dg::ancestry_version(&child) == 1);
    }

    #[test]
    #[expected_failure(abort_code = E_CYCLE_DETECTED, location = social_contracts::derivative_graph)]
    fun test_cycle_a_b_c_a_aborts() {
        let a = id_a();
        let b = id_b();
        let c = id_c();
        let mut b_meta = dg::test_empty_ancestry();
        dg::test_merge_ancestry_for_edge(&mut b_meta, a, &vector[]);
        let mut c_meta = dg::test_empty_ancestry();
        dg::test_merge_ancestry_for_edge(
            &mut c_meta,
            b,
            dg::ancestor_ids(&b_meta),
        );
        dg::test_assert_valid_parent_child_edge(&c, &a, dg::ancestor_ids(&c_meta));
    }

    #[test]
    #[expected_failure(abort_code = E_SELF_EDGE, location = social_contracts::derivative_graph)]
    fun test_self_edge_aborts() {
        let a = id_a();
        dg::test_assert_valid_parent_child_edge(&a, &a, &vector[]);
    }

    #[test]
    #[expected_failure(abort_code = E_ANCESTOR_LIMIT, location = social_contracts::derivative_graph)]
    fun test_ancestor_limit_aborts() {
        let mut ids = vector[];
        let mut i = 0;
        while (i < dg::max_ancestors()) {
            vector::push_back(&mut ids, address::from_u256((i + 1) as u256).to_id());
            i = i + 1;
        };
        dg::test_canonical_union(&ids, &vector[], option::some(address::from_u256(999).to_id()));
    }
}
