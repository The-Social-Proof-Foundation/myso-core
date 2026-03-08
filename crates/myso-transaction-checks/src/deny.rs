// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use fastcrypto_zkp::bn254::zk_login::OIDCProvider;
use move_core_types::ident_str;
use move_core_types::language_storage::TypeTag;
use myso_config::{
    dynamic_transaction_signing_checks::DynamicCheckRunnerError,
    transaction_deny_config::TransactionDenyConfig,
};
use myso_types::{MYSO_FRAMEWORK_ADDRESS, MYSO_SOCIAL_ADDRESS};
use myso_types::{
    base_types::{MySoAddress, ObjectRef},
    error::{MySoError, MySoErrorKind, MySoResult, UserInputError},
    object::Owner,
    signature::GenericSignature,
    storage::{BackingPackageStore, ObjectStore},
    transaction::{Command, InputObjectKind, TransactionData, TransactionDataAPI},
};
use tracing::{error, trace, warn};
macro_rules! deny_if_true {
    ($cond:expr, $msg:expr) => {
        if ($cond) {
            return Err(MySoError(Box::new(MySoErrorKind::UserInputError {
                error: UserInputError::TransactionDenied {
                    error: $msg.to_string(),
                },
            })));
        }
    };
}

/// Check that the provided transaction is allowed to be signed according to the
/// deny config.
pub fn check_transaction_for_signing(
    tx_data: &TransactionData,
    tx_signatures: &[GenericSignature],
    input_object_kinds: &[InputObjectKind],
    receiving_objects: &[ObjectRef],
    filter_config: &TransactionDenyConfig,
    package_store: &dyn BackingPackageStore,
    object_store: &dyn ObjectStore,
) -> MySoResult {
    check_disabled_features(
        filter_config,
        tx_data,
        tx_signatures,
        input_object_kinds,
        package_store,
        object_store,
    )?;

    check_signers(filter_config, tx_data)?;

    check_input_objects(filter_config, input_object_kinds)?;

    check_package_dependencies(filter_config, tx_data, package_store)?;

    check_receiving_objects(filter_config, receiving_objects)?;

    // NB: Only performed at signing time.
    dynamic_transaction_checks(
        filter_config,
        tx_data,
        tx_signatures,
        input_object_kinds,
        receiving_objects,
    )?;

    Ok(())
}

fn dynamic_transaction_checks(
    filter_config: &TransactionDenyConfig,
    tx_data: &TransactionData,
    tx_signatures: &[GenericSignature],
    input_object_kinds: &[InputObjectKind],
    receiving_objects: &[ObjectRef],
) -> MySoResult {
    let Some(dynamic_check) = filter_config.dynamic_transaction_checks() else {
        return Ok(());
    };
    match dynamic_check.run_predicate(
        tx_data,
        tx_signatures,
        input_object_kinds,
        receiving_objects,
    ) {
        // Predicate passed
        Ok(()) => Ok(()),
        // Predicate failed
        Err(DynamicCheckRunnerError::CheckFailure) => {
            warn!(
                "Dynamic transaction predicate rejected transaction: {:?}",
                tx_data.digest()
            );
            Err(MySoErrorKind::UserInputError {
                error: UserInputError::TransactionDenied {
                    error: "Dynamic transaction predicate failed".to_string(),
                },
            }
            .into())
        }
        // Non-predicate failure, so be conservative and deny the transaction.
        Err(e) => {
            error!(
                "Dynamic transaction predicate failed with error: {:?} on transaction: {}. \
                 Rejecting transaction.",
                e,
                tx_data.digest()
            );
            Err(MySoErrorKind::UserInputError {
                error: UserInputError::TransactionDenied {
                    error: e.to_string(),
                },
            }
            .into())
        }
    }
}

fn check_receiving_objects(
    filter_config: &TransactionDenyConfig,
    receiving_objects: &[ObjectRef],
) -> MySoResult {
    deny_if_true!(
        filter_config.receiving_objects_disabled() && !receiving_objects.is_empty(),
        "Receiving objects is temporarily disabled".to_string()
    );
    for (id, _, _) in receiving_objects {
        deny_if_true!(
            filter_config.get_object_deny_set().contains(id),
            format!("Access to object {:?} is temporarily disabled", id)
        );
    }
    Ok(())
}

fn check_disabled_features(
    filter_config: &TransactionDenyConfig,
    tx_data: &TransactionData,
    tx_signatures: &[GenericSignature],
    input_object_kinds: &[InputObjectKind],
    _package_store: &dyn BackingPackageStore,
    object_store: &dyn ObjectStore,
) -> MySoResult {
    deny_if_true!(
        filter_config.user_transaction_disabled(),
        "Transaction signing is temporarily disabled"
    );

    tx_signatures.iter().try_for_each(|s| {
        if let GenericSignature::ZkLoginAuthenticator(z) = s {
            deny_if_true!(
                filter_config.zklogin_sig_disabled(),
                "zkLogin authenticator is temporarily disabled"
            );
            deny_if_true!(
                filter_config.zklogin_disabled_providers().contains(
                    &OIDCProvider::from_iss(z.get_iss())
                        .map_err(|_| MySoError::from(MySoErrorKind::UnexpectedMessage(
                            z.get_iss().to_string()
                        )))?
                        .to_string()
                ),
                "zkLogin OAuth provider is temporarily disabled"
            )
        }
        Ok(())
    })?;

    // Check if publish/upgrade restrictions apply
    let publish_disabled = filter_config.package_publish_disabled();
    let upgrade_disabled = filter_config.package_upgrade_disabled();

    if !publish_disabled && !upgrade_disabled {
        return Ok(());
    }

    // Check if sender owns an UpgradeAdminCap or PackagePublishingAdminCap (admin bypass)
    let sender = tx_data.sender();
    let sender_has_upgrade_admin_cap =
        has_upgrade_admin_cap(sender, input_object_kinds, object_store);
    let sender_has_publish_admin_cap =
        has_package_publishing_admin_cap(sender, input_object_kinds, object_store);

    for command in tx_data.kind().iter_commands() {
        // Allow publish if sender owns UpgradeAdminCap or PackagePublishingAdminCap, otherwise check the disable flag
        deny_if_true!(
            publish_disabled
                && !sender_has_upgrade_admin_cap
                && !sender_has_publish_admin_cap
                && matches!(command, Command::Publish(..)),
            "Package publish is temporarily disabled"
        );
        deny_if_true!(
            upgrade_disabled
                && !sender_has_upgrade_admin_cap
                && matches!(command, Command::Upgrade(..)),
            "Package upgrade is temporarily disabled"
        );
    }
    Ok(())
}

fn is_package_publishing_admin_cap_type(type_tag: &TypeTag) -> bool {
    if let TypeTag::Struct(st) = type_tag {
        st.address == MYSO_FRAMEWORK_ADDRESS
            && st.module.as_ident_str() == ident_str!("package")
            && st.name.as_ident_str() == ident_str!("PackagePublishingAdminCap")
    } else {
        false
    }
}

fn is_upgrade_admin_cap_type(type_tag: &TypeTag) -> bool {
    if let TypeTag::Struct(st) = type_tag {
        st.address == MYSO_SOCIAL_ADDRESS
            && st.module.as_ident_str() == ident_str!("upgrade")
            && st.name.as_ident_str() == ident_str!("UpgradeAdminCap")
    } else {
        false
    }
}

/// Check if the sender owns an UpgradeAdminCap from the social contracts package
fn has_upgrade_admin_cap(
    sender: MySoAddress,
    input_object_kinds: &[InputObjectKind],
    object_store: &dyn ObjectStore,
) -> bool {
    for input_kind in input_object_kinds {
        if let InputObjectKind::ImmOrOwnedMoveObject((object_id, _, _)) = input_kind {
            let Some(object) = object_store.get_object(object_id) else {
                trace!("admin cap check: object {} not found", object_id);
                continue;
            };
            let Owner::AddressOwner(owner_addr) = object.owner else {
                trace!(
                    "admin cap check: object {} owner {:?} is not AddressOwner",
                    object_id, object.owner
                );
                continue;
            };
            if owner_addr != sender {
                trace!(
                    "admin cap check: object {} owner {:?} != sender {:?}",
                    object_id, owner_addr, sender
                );
                continue;
            }
            if let Some(move_object) = object.data.try_as_move() {
                let type_tag = TypeTag::from(move_object.type_().clone());
                if is_upgrade_admin_cap_type(&type_tag) {
                    return true;
                }
                trace!(
                    "admin cap check: object {} type {} does not match UpgradeAdminCap",
                    object_id, type_tag
                );
            }
        }
    }
    false
}

/// Check if the sender owns a PackagePublishingAdminCap from the myso framework package
fn has_package_publishing_admin_cap(
    sender: MySoAddress,
    input_object_kinds: &[InputObjectKind],
    object_store: &dyn ObjectStore,
) -> bool {
    for input_kind in input_object_kinds {
        if let InputObjectKind::ImmOrOwnedMoveObject((object_id, _, _)) = input_kind {
            let Some(object) = object_store.get_object(object_id) else {
                trace!("admin cap check: object {} not found", object_id);
                continue;
            };
            let Owner::AddressOwner(owner_addr) = object.owner else {
                trace!(
                    "admin cap check: object {} owner {:?} is not AddressOwner",
                    object_id, object.owner
                );
                continue;
            };
            if owner_addr != sender {
                trace!(
                    "admin cap check: object {} owner {:?} != sender {:?}",
                    object_id, owner_addr, sender
                );
                continue;
            }
            if let Some(move_object) = object.data.try_as_move() {
                let type_tag = TypeTag::from(move_object.type_().clone());
                if is_package_publishing_admin_cap_type(&type_tag) {
                    return true;
                }
                trace!(
                    "admin cap check: object {} type {} does not match PackagePublishingAdminCap",
                    object_id, type_tag
                );
            }
        }
    }
    false
}

fn check_signers(filter_config: &TransactionDenyConfig, tx_data: &TransactionData) -> MySoResult {
    let deny_map = filter_config.get_address_deny_set();
    if deny_map.is_empty() {
        return Ok(());
    }
    for signer in tx_data.required_signers() {
        deny_if_true!(
            deny_map.contains(&signer),
            format!(
                "Access to account address {:?} is temporarily disabled",
                signer
            )
        );
    }
    Ok(())
}

fn check_input_objects(
    filter_config: &TransactionDenyConfig,
    input_object_kinds: &[InputObjectKind],
) -> MySoResult {
    let deny_map = filter_config.get_object_deny_set();
    let shared_object_disabled = filter_config.shared_object_disabled();
    if deny_map.is_empty() && !shared_object_disabled {
        // No need to iterate through the input objects if no relevant policy is set.
        return Ok(());
    }
    for input_object_kind in input_object_kinds {
        let id = input_object_kind.object_id();
        deny_if_true!(
            deny_map.contains(&id),
            format!("Access to input object {:?} is temporarily disabled", id)
        );
        deny_if_true!(
            shared_object_disabled && input_object_kind.is_shared_object(),
            "Usage of shared object in transactions is temporarily disabled"
        );
    }
    Ok(())
}

fn check_package_dependencies(
    filter_config: &TransactionDenyConfig,
    tx_data: &TransactionData,
    package_store: &dyn BackingPackageStore,
) -> MySoResult {
    let deny_map = filter_config.get_package_deny_set();
    if deny_map.is_empty() {
        return Ok(());
    }
    let mut dependencies = vec![];
    for command in tx_data.kind().iter_commands() {
        match command {
            Command::Publish(_, deps) => {
                // It is possible that the deps list is inaccurate since it's provided
                // by the user. But that's OK because this publish transaction will fail
                // to execute in the end. Similar reasoning for Upgrade.
                dependencies.extend(deps.iter().copied());
            }
            Command::Upgrade(_, deps, package_id, _) => {
                dependencies.extend(deps.iter().copied());
                // It's crucial that we don't allow upgrading a package in the deny list,
                // otherwise one can bypass the deny list by upgrading a package.
                dependencies.push(*package_id);
            }
            Command::MoveCall(call) => {
                let package = package_store.get_package_object(&call.package)?.ok_or(
                    MySoErrorKind::UserInputError {
                        error: UserInputError::ObjectNotFound {
                            object_id: call.package,
                            version: None,
                        },
                    },
                )?;
                // linkage_table maps from the original package ID to the upgraded ID for each
                // dependency. Here we only check the upgraded (i.e. the latest) ID against the
                // deny list. This means that we only make sure that the denied package is not
                // currently used as a dependency. This allows us to deny an older version of
                // package but permits the use of a newer version.
                dependencies.extend(
                    package
                        .move_package()
                        .linkage_table()
                        .values()
                        .map(|upgrade_info| upgrade_info.upgraded_id),
                );
                dependencies.push(package.move_package().id());
            }
            Command::TransferObjects(..)
            | &Command::SplitCoins(..)
            | &Command::MergeCoins(..)
            | &Command::MakeMoveVec(..) => {}
        }
    }
    for dep in dependencies {
        deny_if_true!(
            deny_map.contains(&dep),
            format!("Access to package {:?} is temporarily disabled", dep)
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{is_package_publishing_admin_cap_type, is_upgrade_admin_cap_type};
    use move_core_types::ident_str;
    use move_core_types::language_storage::{StructTag, TypeTag};
    use myso_types::{MYSO_FRAMEWORK_ADDRESS, MYSO_SOCIAL_ADDRESS};

    #[test]
    fn test_is_package_publishing_admin_cap_type_positive() {
        let tag = TypeTag::Struct(Box::new(StructTag {
            address: MYSO_FRAMEWORK_ADDRESS,
            module: ident_str!("package").to_owned(),
            name: ident_str!("PackagePublishingAdminCap").to_owned(),
            type_params: vec![],
        }));
        assert!(is_package_publishing_admin_cap_type(&tag));
    }

    #[test]
    fn test_is_package_publishing_admin_cap_type_negative_wrong_module() {
        let tag = TypeTag::Struct(Box::new(StructTag {
            address: MYSO_FRAMEWORK_ADDRESS,
            module: ident_str!("other").to_owned(),
            name: ident_str!("PackagePublishingAdminCap").to_owned(),
            type_params: vec![],
        }));
        assert!(!is_package_publishing_admin_cap_type(&tag));
    }

    #[test]
    fn test_is_package_publishing_admin_cap_type_negative_wrong_name() {
        let tag = TypeTag::Struct(Box::new(StructTag {
            address: MYSO_FRAMEWORK_ADDRESS,
            module: ident_str!("package").to_owned(),
            name: ident_str!("Other").to_owned(),
            type_params: vec![],
        }));
        assert!(!is_package_publishing_admin_cap_type(&tag));
    }

    #[test]
    fn test_is_package_publishing_admin_cap_type_negative_wrong_address() {
        let tag = TypeTag::Struct(Box::new(StructTag {
            address: MYSO_SOCIAL_ADDRESS,
            module: ident_str!("package").to_owned(),
            name: ident_str!("PackagePublishingAdminCap").to_owned(),
            type_params: vec![],
        }));
        assert!(!is_package_publishing_admin_cap_type(&tag));
    }

    #[test]
    fn test_is_package_publishing_admin_cap_type_negative_not_struct() {
        assert!(!is_package_publishing_admin_cap_type(&TypeTag::Bool));
    }

    #[test]
    fn test_is_upgrade_admin_cap_type_positive() {
        let tag = TypeTag::Struct(Box::new(StructTag {
            address: MYSO_SOCIAL_ADDRESS,
            module: ident_str!("upgrade").to_owned(),
            name: ident_str!("UpgradeAdminCap").to_owned(),
            type_params: vec![],
        }));
        assert!(is_upgrade_admin_cap_type(&tag));
    }

    #[test]
    fn test_is_upgrade_admin_cap_type_negative_wrong_module() {
        let tag = TypeTag::Struct(Box::new(StructTag {
            address: MYSO_SOCIAL_ADDRESS,
            module: ident_str!("other").to_owned(),
            name: ident_str!("UpgradeAdminCap").to_owned(),
            type_params: vec![],
        }));
        assert!(!is_upgrade_admin_cap_type(&tag));
    }

    #[test]
    fn test_is_upgrade_admin_cap_type_negative_wrong_address() {
        let tag = TypeTag::Struct(Box::new(StructTag {
            address: MYSO_FRAMEWORK_ADDRESS,
            module: ident_str!("upgrade").to_owned(),
            name: ident_str!("UpgradeAdminCap").to_owned(),
            type_params: vec![],
        }));
        assert!(!is_upgrade_admin_cap_type(&tag));
    }
}
