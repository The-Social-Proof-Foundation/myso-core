// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use fastcrypto::traits::KeyPair;
use move_core_types::{account_address::AccountAddress, ident_str};
use myso_config::genesis::Genesis;
use myso_macros::nondeterministic;
use myso_types::base_types::{FullObjectRef, ObjectID, random_object_ref};
use myso_types::crypto::AuthorityKeyPair;
use myso_types::crypto::{AccountKeyPair, AuthorityPublicKeyBytes, Signer};
use myso_types::effects::TestEffectsBuilder;
use myso_types::signature_verification::VerifiedDigestCache;
use myso_types::transaction::ObjectArg;
use myso_types::transaction::{
    CallArg, SignedTransaction, TEST_ONLY_GAS_UNIT_FOR_TRANSFER, Transaction, TransactionData,
};
use myso_types::utils::create_fake_transaction;
use myso_types::utils::to_sender_signed_transaction;
use myso_types::{
    base_types::{AuthorityName, ExecutionDigests, MySoAddress, ObjectRef, TransactionDigest},
    committee::Committee,
    crypto::{AuthoritySignInfo, AuthoritySignature},
    message_envelope::Message,
    transaction::CertifiedTransaction,
};
use shared_crypto::intent::{Intent, IntentScope};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;
use tracing::{info, warn};

use crate::authority::AuthorityState;

// note: clippy is confused about this being dead - it appears to only be used in cfg(test), but
// adding #[cfg(test)] causes other targets to fail
#[allow(dead_code)]
pub(crate) fn init_state_parameters_from_rng<R>(rng: &mut R) -> (Genesis, AuthorityKeyPair)
where
    R: rand::CryptoRng + rand::RngCore,
{
    let dir = nondeterministic!(tempfile::TempDir::new().unwrap());
    let network_config = myso_swarm_config::network_config_builder::ConfigBuilder::new(&dir)
        .rng(rng)
        .build();
    let genesis = network_config.genesis;
    let authority_key = network_config.validator_configs[0]
        .protocol_key_pair()
        .copy();

    (genesis, authority_key)
}

pub async fn wait_for_tx(digest: TransactionDigest, state: Arc<AuthorityState>, delay: Duration) {
    match timeout(
        delay,
        state
            .get_transaction_cache_reader()
            .notify_read_executed_effects("", &[digest]),
    )
    .await
    {
        Ok(_) => info!(?digest, "digest found"),
        Err(e) => {
            warn!(?digest, "digest not found!");
            panic!("timed out waiting for effects of digest! {e}");
        }
    }
}

pub fn create_fake_cert_and_effect_digest<'a>(
    signers: impl Iterator<
        Item = (
            &'a AuthorityName,
            &'a (dyn Signer<AuthoritySignature> + Send + Sync),
        ),
    >,
    committee: &Committee,
) -> (ExecutionDigests, CertifiedTransaction) {
    let transaction = create_fake_transaction();
    let cert = CertifiedTransaction::new(
        transaction.data().clone(),
        signers
            .map(|(name, signer)| {
                AuthoritySignInfo::new(
                    committee.epoch,
                    transaction.data(),
                    Intent::myso_app(IntentScope::SenderSignedTransaction),
                    *name,
                    signer,
                )
            })
            .collect(),
        committee,
    )
    .unwrap();
    let effects = TestEffectsBuilder::new(transaction.data()).build();
    (
        ExecutionDigests::new(*transaction.digest(), effects.digest()),
        cert,
    )
}

pub fn make_transfer_myso_transaction(
    gas_object: ObjectRef,
    recipient: MySoAddress,
    amount: Option<u64>,
    sender: MySoAddress,
    keypair: &AccountKeyPair,
    gas_price: u64,
) -> Transaction {
    let data = TransactionData::new_transfer_myso(
        recipient,
        sender,
        amount,
        gas_object,
        gas_price * TEST_ONLY_GAS_UNIT_FOR_TRANSFER,
        gas_price,
    );
    to_sender_signed_transaction(data, keypair)
}

pub fn make_pay_myso_transaction(
    gas_object: ObjectRef,
    coins: Vec<ObjectRef>,
    recipients: Vec<MySoAddress>,
    amounts: Vec<u64>,
    sender: MySoAddress,
    keypair: &AccountKeyPair,
    gas_price: u64,
    gas_budget: u64,
) -> Transaction {
    let data = TransactionData::new_pay_myso(
        sender, coins, recipients, amounts, gas_object, gas_budget, gas_price,
    )
    .unwrap();
    to_sender_signed_transaction(data, keypair)
}

pub fn make_transfer_object_transaction(
    object_ref: ObjectRef,
    gas_object: ObjectRef,
    sender: MySoAddress,
    keypair: &AccountKeyPair,
    recipient: MySoAddress,
    gas_price: u64,
) -> Transaction {
    let data = TransactionData::new_transfer(
        recipient,
        FullObjectRef::from_fastpath_ref(object_ref),
        sender,
        gas_object,
        gas_price * TEST_ONLY_GAS_UNIT_FOR_TRANSFER * 10,
        gas_price,
    );
    to_sender_signed_transaction(data, keypair)
}

pub fn make_transfer_object_move_transaction(
    src: MySoAddress,
    keypair: &AccountKeyPair,
    dest: MySoAddress,
    object_ref: ObjectRef,
    framework_obj_id: ObjectID,
    gas_object_ref: ObjectRef,
    gas_budget_in_units: u64,
    gas_price: u64,
) -> Transaction {
    let args = vec![
        CallArg::Object(ObjectArg::ImmOrOwnedObject(object_ref)),
        CallArg::Pure(bcs::to_bytes(&AccountAddress::from(dest)).unwrap()),
    ];

    to_sender_signed_transaction(
        TransactionData::new_move_call(
            src,
            framework_obj_id,
            ident_str!("object_basics").to_owned(),
            ident_str!("transfer").to_owned(),
            Vec::new(),
            gas_object_ref,
            args,
            gas_budget_in_units * gas_price,
            gas_price,
        )
        .unwrap(),
        keypair,
    )
}

/// Make a dummy tx that uses random object refs.
pub fn make_dummy_tx(
    receiver: MySoAddress,
    sender: MySoAddress,
    sender_sec: &AccountKeyPair,
) -> Transaction {
    Transaction::from_data_and_signer(
        TransactionData::new_transfer(
            receiver,
            FullObjectRef::from_fastpath_ref(random_object_ref()),
            sender,
            random_object_ref(),
            TEST_ONLY_GAS_UNIT_FOR_TRANSFER * 10,
            10,
        ),
        vec![sender_sec],
    )
}

/// Make a cert using an arbitrarily large committee.
pub fn make_cert_with_large_committee(
    committee: &Committee,
    key_pairs: &[AuthorityKeyPair],
    transaction: &Transaction,
) -> CertifiedTransaction {
    // assumes equal weighting.
    let len = committee.voting_rights.len();
    assert_eq!(len, key_pairs.len());
    let count = (len * 2).div_ceil(3);

    let sigs: Vec<_> = key_pairs
        .iter()
        .take(count)
        .map(|key_pair| {
            SignedTransaction::new(
                committee.epoch(),
                transaction.clone().into_data(),
                key_pair,
                AuthorityPublicKeyBytes::from(key_pair.public()),
            )
            .auth_sig()
            .clone()
        })
        .collect();

    let cert = CertifiedTransaction::new(transaction.clone().into_data(), sigs, committee).unwrap();
    cert.verify_signatures_authenticated(
        committee,
        &Default::default(),
        Arc::new(VerifiedDigestCache::new_empty()),
    )
    .unwrap();
    cert
}
