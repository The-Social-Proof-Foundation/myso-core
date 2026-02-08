// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::payload::rpc_command_processor::DEFAULT_GAS_BUDGET;
use crate::payload::{PayMySo, ProcessPayload, RpcCommandProcessor, SignerInfo};
use async_trait::async_trait;
use futures::future::join_all;
use myso_types::base_types::MySoAddress;
use myso_types::crypto::{EncodeDecodeBase64, MySoKeyPair};
use myso_types::transaction::TransactionData;
use myso_types::transaction_driver_types::ExecuteTransactionRequestType;
use tracing::debug;

#[async_trait]
impl<'a> ProcessPayload<'a, &'a PayMySo> for RpcCommandProcessor {
    async fn process(
        &'a self,
        _op: &'a PayMySo,
        signer_info: &Option<SignerInfo>,
    ) -> anyhow::Result<()> {
        let clients = self.get_clients().await?;
        let SignerInfo {
            encoded_keypair,
            gas_budget,
            gas_payment,
        } = signer_info.clone().unwrap();
        let recipient = MySoAddress::random_for_testing_only();
        let amount = 1;
        let gas_budget = gas_budget.unwrap_or(DEFAULT_GAS_BUDGET);
        let gas_payments = gas_payment.unwrap();

        let keypair =
            MySoKeyPair::decode_base64(&encoded_keypair).expect("Decoding keypair should not fail");

        debug!(
            "Transfer MySo {} time to {recipient} with {amount} MIST with {gas_payments:?}",
            gas_payments.len()
        );

        let sender = MySoAddress::from(&keypair.public());
        // TODO: For write operations, we usually just want to submit the transaction to fullnode
        // Let's figure out what's the best way to support other mode later
        let client = clients.first().unwrap();
        let gas_price = client
            .governance_api()
            .get_reference_gas_price()
            .await
            .expect("Unable to fetch gas price");
        join_all(gas_payments.iter().map(|gas| async {
            let tx = TransactionData::new_transfer_myso(
                recipient,
                sender,
                Some(amount),
                self.get_object_ref(client, gas).await,
                gas_budget,
                gas_price,
            );
            self.sign_and_execute(
                client,
                &keypair,
                tx,
                ExecuteTransactionRequestType::WaitForEffectsCert,
            )
            .await
        }))
        .await;

        Ok(())
    }
}
