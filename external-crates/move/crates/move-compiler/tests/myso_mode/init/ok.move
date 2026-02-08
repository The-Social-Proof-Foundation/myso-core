// valid init function
module a::m {
    use myso::tx_context;

    fun init(_: &mut tx_context::TxContext) {}
}

module myso::tx_context {
    struct TxContext has drop {}
}
