// init functions cannot be public, cannot have entry, cannot be public(friend)
module a::m0 {
    use myso::tx_context;

    public fun init(_ctx: &mut tx_context::TxContext) {
        abort 0
    }
}

module a::m1 {
    use myso::tx_context;

    entry fun init(_ctx: &mut tx_context::TxContext) {
        abort 0
    }
}

module a::m2 {
    use myso::tx_context;

    public(friend) fun init(_ctx: &mut tx_context::TxContext) {
        abort 0
    }
}

module myso::tx_context {
    struct TxContext has drop {}
}
