// init is unused but does not error because we are in MySo mode
module a::m {
    fun init(_: &mut myso::tx_context::TxContext) {}
}

module myso::tx_context {
    struct TxContext has drop {}
}
