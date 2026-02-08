// init functions cannot have generics
module a::m {
    use myso::tx_context;

    fun init<T>(_ctx: &mut tx_context::TxContext) {
        abort 0
    }
}

module myso::tx_context {
    struct TxContext has drop {}
}
