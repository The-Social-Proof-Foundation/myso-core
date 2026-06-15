/// Module: permissioned_group_display
///
/// Display support for `PermissionedGroup<T>` types.
///
/// Since `PermissionedGroup<T>` is defined in `permissioned_group`, extending
/// packages cannot directly create `Display<PermissionedGroup<T>>`.
///
/// This module provides a shared `PermissionedGroupPublisher` that holds the
/// framework Publisher. Extending packages can call `setup_display<T>`
/// with their own Publisher to create `Display<PermissionedGroup<T>>`.
module myso::permissioned_group_display;

use myso::permissioned_group::PermissionedGroup;
use std::string::String;
use myso::display;
use myso::package::{Self, Publisher};

// === Error Codes ===

/// Type T is not from the same module as the publisher
const ETypeNotFromModule: u64 = 0;

// === One-Time Witness ===

/// OTW for claiming Publisher and initializing PermissionedGroupPublisher.
public struct PERMISSIONED_GROUP_DISPLAY() has drop;

// === Structs ===

/// Shared object holding the framework Publisher for permissioned groups.
/// Used by extending packages to create `Display<PermissionedGroup<T>>`.
public struct PermissionedGroupPublisher has key {
    id: UID,
    publisher: Publisher,
}

// === Init ===

fun init(otw: PERMISSIONED_GROUP_DISPLAY, ctx: &mut TxContext) {
    transfer::share_object(PermissionedGroupPublisher {
        id: object::new(ctx),
        publisher: package::claim(otw, ctx),
    });
}

// === Public Functions ===

/// Creates a `Display<PermissionedGroup<T>>` using the shared publisher.
/// The caller must provide their own Publisher to prove they own the module
/// that defines type T. The Display is transferred to the transaction sender.
#[allow(lint(self_transfer))]
public fun setup_display<T: drop>(
    pg_publisher: &PermissionedGroupPublisher,
    publisher: &Publisher,
    name: String,
    description: String,
    image_url: String,
    project_url: String,
    link: String,
    ctx: &mut TxContext,
) {
    assert!(publisher.from_module<T>(), ETypeNotFromModule);

    let mut display = display::new<PermissionedGroup<T>>(&pg_publisher.publisher, ctx);

    display.add(b"name".to_string(), name);
    display.add(b"description".to_string(), description);
    display.add(b"creator".to_string(), b"{creator}".to_string());
    display.add(b"image_url".to_string(), image_url);
    display.add(b"project_url".to_string(), project_url);
    display.add(b"link".to_string(), link);

    display.update_version();
    transfer::public_transfer(display, ctx.sender());
}

// === Test Helpers ===

#[test_only]
public fun init_for_testing(ctx: &mut TxContext) {
    transfer::share_object(PermissionedGroupPublisher {
        id: object::new(ctx),
        publisher: package::test_claim(PERMISSIONED_GROUP_DISPLAY(), ctx),
    });
}
