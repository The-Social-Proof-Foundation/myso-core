module a::m {
    use myso::tx_context;

    public entry fun foo<T>(_: T, _: &mut tx_context::TxContext) {
        abort 0
    }
}

module myso::tx_context {
    struct TxContext has drop {}
}
