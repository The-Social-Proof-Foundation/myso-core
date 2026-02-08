// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use bincode::Decode;
use bincode::Encode;
use move_core_types::language_storage::TypeTag;
use myso_indexer_alt_framework::types::base_types::MySoAddress;

#[derive(Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) struct Key {
    #[bincode(with_serde)]
    pub(crate) owner: MySoAddress,

    /// The inner type of some balance `Balance<T>`, e.g. for `0x2::balance::Balance<0x2::myso::MYSO>`
    /// this would be `0x2::myso::MYSO`.
    #[bincode(with_serde)]
    pub(crate) type_: TypeTag,
}

/// Options for creating this index's column family in RocksDB.
pub(crate) fn options(base_options: &rocksdb::Options) -> rocksdb::Options {
    base_options.clone()
}
