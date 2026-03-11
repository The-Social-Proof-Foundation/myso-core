// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Reusable Merkle proof verification for claim settlement.
/// Tree construction happens offchain; onchain only verifies proofs.
/// Uses Blake2b-256 for hashing, aligned with accumulator_settlement pattern.

module mydata::merkle {
    use std::vector;
    use myso::hash;
    use myso::bcs;

    const HASH_LEN: u64 = 32;

    /// Verify that a leaf is included in a Merkle tree with the given root.
    /// @param leaf - Hash of the leaf (e.g. from leaf_hash)
    /// @param proof - Sibling hashes from leaf level up to root (excluding root)
    /// @param leaf_index - Index of the leaf in the tree (0 = leftmost)
    /// @param root - Expected Merkle root
    public fun verify_proof(
        leaf: vector<u8>,
        proof: &vector<vector<u8>>,
        leaf_index: u64,
        root: vector<u8>,
    ): bool {
        assert!(vector::length(&leaf) == HASH_LEN, 0);
        assert!(vector::length(&root) == HASH_LEN, 1);

        let mut current = leaf;
        let mut idx = leaf_index;
        let len = vector::length(proof);

        let mut i = 0u64;
        while (i < len) {
            let sibling = vector::borrow(proof, i);
            assert!(vector::length(sibling) == HASH_LEN, 2);

            current = if (idx % 2 == 0) {
                hash_pair(&current, sibling)
            } else {
                hash_pair(sibling, &current)
            };
            idx = idx / 2;
            i = i + 1;
        };

        current == root
    }

    /// Hash two 32-byte values for Merkle tree internal node.
    /// Order matters: hash(left || right).
    fun hash_pair(left: &vector<u8>, right: &vector<u8>): vector<u8> {
        let mut concat = *left;
        vector::append(&mut concat, *right);
        hash::blake2b256(&concat)
    }

    /// Construct leaf hash for claim verification.
    /// Leaf = blake2b256(address || amount || snapshot_id).
    public fun leaf_hash(
        addr: address,
        amount: u64,
        snapshot_id: vector<u8>,
    ): vector<u8> {
        let mut data = bcs::to_bytes(&addr);
        vector::append(&mut data, bcs::to_bytes(&amount));
        vector::append(&mut data, snapshot_id);
        hash::blake2b256(&data)
    }

    #[test]
    fun test_merkle_verify_proof() {
        let leaf = hash::blake2b256(&b"leaf0");
        let leaf1 = hash::blake2b256(&b"leaf1");
        let mut concat = leaf;
        vector::append(&mut concat, leaf1);
        let parent = hash::blake2b256(&concat);
        let leaf_for_verify = hash::blake2b256(&b"leaf0");
        let proof = vector[leaf1];
        assert!(verify_proof(leaf_for_verify, &proof, 0, parent), 0);
    }

    #[test]
    fun test_merkle_reject_invalid_proof() {
        let leaf = hash::blake2b256(&b"leaf0");
        let wrong_sibling = hash::blake2b256(&b"wrong");
        let proof = vector[wrong_sibling];
        let root = hash::blake2b256(&b"tampered");
        assert!(!verify_proof(leaf, &proof, 0, root), 0);
    }
}
