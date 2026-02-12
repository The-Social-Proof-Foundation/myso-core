// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;

use myso_json_rpc_types::{DelegatedStake, MySoCommittee, ValidatorApys};
use myso_open_rpc_macros::open_rpc;
use myso_types::base_types::{ObjectID, MySoAddress};
use myso_types::myso_serde::BigInt;
use myso_types::myso_system_state::myso_system_state_summary::MySoSystemStateSummary;

#[open_rpc(namespace = "mysox", tag = "Governance Read API")]
#[rpc(server, client, namespace = "mysox")]
pub trait GovernanceReadApi {
    /// Return one or more [DelegatedStake]. If a Stake was withdrawn its status will be Unstaked.
    #[method(name = "getStakesByIds")]
    async fn get_stakes_by_ids(
        &self,
        staked_myso_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>>;

    /// Return all [DelegatedStake].
    #[method(name = "getStakes")]
    async fn get_stakes(&self, owner: MySoAddress) -> RpcResult<Vec<DelegatedStake>>;

    /// Return the committee information for the asked `epoch`.
    #[method(name = "getCommitteeInfo")]
    async fn get_committee_info(
        &self,
        /// The epoch of interest. If None, default to the latest epoch
        epoch: Option<BigInt<u64>>,
    ) -> RpcResult<MySoCommittee>;

    /// Return the latest MYSO system state object on-chain.
    #[method(name = "getLatestMySoSystemState")]
    async fn get_latest_myso_system_state(&self) -> RpcResult<MySoSystemStateSummary>;

    /// Return the reference gas price for the network
    #[method(name = "getReferenceGasPrice")]
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>>;

    /// Return the validator APY
    #[method(name = "getValidatorsApy")]
    async fn get_validators_apy(&self) -> RpcResult<ValidatorApys>;
}
