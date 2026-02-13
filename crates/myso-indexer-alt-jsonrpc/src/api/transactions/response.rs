// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use anyhow::Context as _;
use futures::future::OptionFuture;
use move_core_types::annotated_value::MoveDatatypeLayout;
use move_core_types::annotated_value::MoveTypeLayout;
use myso_indexer_alt_reader::kv_loader::TransactionContents;
use myso_indexer_alt_reader::objects::VersionedObjectKey;
use myso_indexer_alt_reader::tx_balance_changes::TxBalanceChangeKey;
use myso_indexer_alt_schema::transactions::BalanceChange;
use myso_indexer_alt_schema::transactions::StoredTxBalanceChange;
use myso_json_rpc_types::BalanceChange as MySoBalanceChange;
use myso_json_rpc_types::MySoEvent;
use myso_json_rpc_types::MySoTransactionBlock;
use myso_json_rpc_types::MySoTransactionBlockData;
use myso_json_rpc_types::MySoTransactionBlockEffects;
use myso_json_rpc_types::MySoTransactionBlockEvents;
use myso_json_rpc_types::MySoTransactionBlockResponse;
use myso_json_rpc_types::MySoTransactionBlockResponseOptions;
use myso_json_rpc_types::ObjectChange as MySoObjectChange;
use myso_types::TypeTag;
use myso_types::base_types::ObjectID;
use myso_types::base_types::SequenceNumber;
use myso_types::digests::ObjectDigest;
use myso_types::digests::TransactionDigest;
use myso_types::effects::IDOperation;
use myso_types::effects::ObjectChange;
use myso_types::effects::TransactionEffects;
use myso_types::effects::TransactionEffectsAPI;
use myso_types::event::Event;
use myso_types::object::Object;
use myso_types::signature::GenericSignature;
use myso_types::transaction::TransactionData;
use myso_types::transaction::TransactionDataAPI;
use tokio::join;

use crate::api::transactions::error::Error;
use crate::context::Context;
use crate::error::RpcError;
use crate::error::invalid_params;
use crate::error::rpc_bail;

/// Fetch the necessary data from the stores in `ctx` and transform it to build a response for the
/// transaction identified by `digest`, according to the response `options`.
pub(super) async fn transaction(
    ctx: &Context,
    digest: TransactionDigest,
    options: &MySoTransactionBlockResponseOptions,
) -> Result<MySoTransactionBlockResponse, RpcError<Error>> {
    let tx = ctx.kv_loader().load_one_transaction(digest);
    let stored_bc: OptionFuture<_> = options
        .show_balance_changes
        .then(|| ctx.pg_loader().load_one(TxBalanceChangeKey(digest)))
        .into();

    let (tx, stored_bc) = join!(tx, stored_bc);

    let tx = tx
        .context("Failed to fetch transaction from store")?
        .ok_or_else(|| invalid_params(Error::NotFound(digest)))?;

    // Balance changes might not be present because of pruning, in which case we return
    // nothing, even if the changes were requested.
    let stored_bc = match stored_bc
        .transpose()
        .context("Failed to fetch balance changes from store")?
    {
        Some(None) => return Err(invalid_params(Error::BalanceChangesNotFound(digest))),
        Some(changes) => changes,
        None => None,
    };

    let digest = tx.digest()?;

    let mut response = MySoTransactionBlockResponse::new(digest);

    response.timestamp_ms = tx.timestamp_ms();
    response.checkpoint = tx.cp_sequence_number();

    if options.show_input {
        response.transaction = Some(input(ctx, &tx).await?);
    }

    if options.show_raw_input {
        response.raw_transaction = tx.raw_transaction()?;
    }

    if options.show_effects {
        response.effects = Some(effects(&tx)?);
    }

    if options.show_raw_effects {
        response.raw_effects = tx.raw_effects()?;
    }

    if options.show_events {
        response.events = Some(events(ctx, digest, &tx).await?);
    }

    if let Some(changes) = stored_bc {
        response.balance_changes = Some(balance_changes(changes)?);
    }

    if options.show_object_changes {
        response.object_changes = Some(object_changes(ctx, digest, &tx).await?);
    }

    Ok(response)
}

/// Extract a representation of the transaction's input data from the stored form.
async fn input(
    ctx: &Context,
    tx: &TransactionContents,
) -> Result<MySoTransactionBlock, RpcError<Error>> {
    let data: TransactionData = tx.data()?;
    let tx_signatures: Vec<GenericSignature> = tx.signatures()?;

    Ok(MySoTransactionBlock {
        data: MySoTransactionBlockData::try_from_with_package_resolver(
            data,
            ctx.package_resolver(),
        )
        .await
        .context("Failed to resolve types in transaction data")?,
        tx_signatures,
    })
}

/// Extract a representation of the transaction's effects from the stored form.
fn effects(tx: &TransactionContents) -> Result<MySoTransactionBlockEffects, RpcError<Error>> {
    let effects: TransactionEffects = tx.effects()?;
    Ok(effects
        .try_into()
        .context("Failed to convert Effects into response")?)
}

/// Extract the transaction's events from its stored form.
async fn events(
    ctx: &Context,
    digest: TransactionDigest,
    tx: &TransactionContents,
) -> Result<MySoTransactionBlockEvents, RpcError<Error>> {
    let events: Vec<Event> = tx.events()?;
    let mut myso_events = Vec::with_capacity(events.len());

    for (ix, event) in events.into_iter().enumerate() {
        let layout = match ctx
            .package_resolver()
            .type_layout(event.type_.clone().into())
            .await
            .with_context(|| {
                format!(
                    "Failed to resolve layout for {}",
                    event.type_.to_canonical_display(/* with_prefix */ true)
                )
            })? {
            MoveTypeLayout::Struct(s) => MoveDatatypeLayout::Struct(s),
            MoveTypeLayout::Enum(e) => MoveDatatypeLayout::Enum(e),
            _ => rpc_bail!(
                "Event {ix} is not a struct or enum: {}",
                event.type_.to_canonical_string(/* with_prefix */ true)
            ),
        };

        let myso_event = MySoEvent::try_from(event, digest, ix as u64, tx.timestamp_ms(), layout)
            .with_context(|| format!("Failed to convert Event {ix} into response"))?;

        myso_events.push(myso_event)
    }

    Ok(MySoTransactionBlockEvents { data: myso_events })
}

/// Extract the transaction's balance changes from their stored form.
fn balance_changes(
    balance_changes: StoredTxBalanceChange,
) -> Result<Vec<MySoBalanceChange>, RpcError<Error>> {
    let balance_changes: Vec<BalanceChange> = bcs::from_bytes(&balance_changes.balance_changes)
        .context("Failed to deserialize BalanceChanges")?;
    let mut response = Vec::with_capacity(balance_changes.len());

    for BalanceChange::V1 {
        owner,
        coin_type,
        amount,
    } in balance_changes
    {
        let coin_type = TypeTag::from_str(&coin_type)
            .with_context(|| format!("Invalid coin type: {coin_type:?}"))?;

        response.push(MySoBalanceChange {
            owner,
            coin_type,
            amount,
        });
    }

    Ok(response)
}

/// Extract the transaction's object changes. Object IDs and versions are fetched from the stored
/// transaction, and the object contents are fetched separately by a data loader.
async fn object_changes(
    ctx: &Context,
    digest: TransactionDigest,
    tx: &TransactionContents,
) -> Result<Vec<MySoObjectChange>, RpcError<Error>> {
    let tx_data: TransactionData = tx.data()?;
    let effects: TransactionEffects = tx.effects()?;

    let mut keys = vec![];
    let native_changes = effects.object_changes();
    for change in &native_changes {
        let id = change.id;
        if let Some(version) = change.input_version {
            keys.push(VersionedObjectKey(id, version.value()));
        }
        if let Some(version) = change.output_version {
            keys.push(VersionedObjectKey(id, version.value()));
        }
    }

    let objects = ctx
        .kv_loader()
        .load_many_objects(keys)
        .await
        .context("Failed to fetch object contents")?;

    // Fetch and deserialize the contents of an object, based on its object ref. Assumes that all
    // object versions that will be fetched in this way have come from a valid transaction, and
    // have been passed to the data loader in the call above. This means that if they cannot be
    // found, they must have been pruned.
    let fetch_object = |id: ObjectID,
                        v: Option<SequenceNumber>,
                        d: Option<ObjectDigest>|
     -> Result<Option<(Object, ObjectDigest)>, RpcError<Error>> {
        let Some(v) = v else { return Ok(None) };
        let Some(d) = d else { return Ok(None) };

        let v = v.value();

        let o = objects
            .get(&VersionedObjectKey(id, v))
            .ok_or_else(|| invalid_params(Error::PrunedObject(digest, id, v)))?;

        Ok(Some((o.clone(), d)))
    };

    let mut changes = Vec::with_capacity(native_changes.len());

    for change in native_changes {
        let &ObjectChange {
            id: object_id,
            id_operation,
            input_version,
            input_digest,
            output_version,
            output_digest,
            ..
        } = &change;

        let input = fetch_object(object_id, input_version, input_digest)?;
        let output = fetch_object(object_id, output_version, output_digest)?;

        use IDOperation as ID;
        changes.push(match (id_operation, input, output) {
            (ID::Created, Some((i, _)), _) => rpc_bail!(
                "Unexpected input version {} for object {object_id} created by transaction {digest}",
                i.version().value(),
            ),

            (ID::Deleted, _, Some((o, _))) => rpc_bail!(
                "Unexpected output version {} for object {object_id} deleted by transaction {digest}",
                o.version().value(),
            ),

            // The following cases don't end up in the output: created and wrapped objects,
            // unwrapped objects (and by extension, unwrapped and deleted objects), system package
            // upgrades (which happen in place).
            (ID::Created, _, None) => continue,
            (ID::None, None, _) => continue,
            (ID::None, _, Some((o, _))) if o.is_package() => continue,
            (ID::Deleted, None, _) => continue,

            (ID::Created, _, Some((o, d))) if o.is_package() => MySoObjectChange::Published {
                package_id: object_id,
                version: o.version(),
                digest: d,
                modules: o
                    .data
                    .try_as_package()
                    .unwrap() // SAFETY: Match guard checks that the object is a package.
                    .serialized_module_map()
                    .keys()
                    .cloned()
                    .collect(),
            },

            (ID::Created, _, Some((o, d))) => MySoObjectChange::Created {
                sender: tx_data.sender(),
                owner: o.owner().clone(),
                object_type: o
                    .struct_tag()
                    .with_context(|| format!("No type for object {object_id}"))?,
                object_id,
                version: o.version(),
                digest: d,
            },

            (ID::None, Some((i, _)), Some((o, od))) if i.owner() != o.owner() => {
                MySoObjectChange::Transferred {
                    sender: tx_data.sender(),
                    recipient: o.owner().clone(),
                    object_type: o
                        .struct_tag()
                        .with_context(|| format!("No type for object {object_id}"))?,
                    object_id,
                    version: o.version(),
                    digest: od,
                }
            }

            (ID::None, Some((i, _)), Some((o, od))) => MySoObjectChange::Mutated {
                sender: tx_data.sender(),
                owner: o.owner().clone(),
                object_type: o
                    .struct_tag()
                    .with_context(|| format!("No type for object {object_id}"))?,
                object_id,
                version: o.version(),
                previous_version: i.version(),
                digest: od,
            },

            (ID::None, Some((i, _)), None) => MySoObjectChange::Wrapped {
                sender: tx_data.sender(),
                object_type: i
                    .struct_tag()
                    .with_context(|| format!("No type for object {object_id}"))?,
                object_id,
                version: effects.lamport_version(),
            },

            (ID::Deleted, Some((i, _)), None) => MySoObjectChange::Deleted {
                sender: tx_data.sender(),
                object_type: i
                    .struct_tag()
                    .with_context(|| format!("No type for object {object_id}"))?,
                object_id,
                version: effects.lamport_version(),
            },
        })
    }

    Ok(changes)
}
