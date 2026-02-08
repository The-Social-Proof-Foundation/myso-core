// valid because we can use `derived_object::claim` without triggering id leak
module a::m {
    use myso::derived_object;
    use myso::object;

    struct A has key {
        id: object::UID,
    }

    public fun no_leak(ctx: &mut myso::tx_context::TxContext): A {
        A {
            id: derived_object::claim(object::new(ctx), 0u64),
        }
    }
}

module myso::object {
    struct UID has store {
        id: address,
    }

    public fun new(_: &mut myso::tx_context::TxContext): UID {
        abort 0
    }
}

module myso::tx_context {
    struct TxContext has drop {}
}

module myso::derived_object {
    use myso::object::UID;

    public fun claim<T: copy + store + drop>(_: UID, _: T): UID {
        abort 0
    }
}
