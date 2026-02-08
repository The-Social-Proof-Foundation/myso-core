// invalid, no one-time witness type parameter in init

module a::m {
    use myso::tx_context;

    struct M has drop { dummy: bool }

    fun init(_ctx: &mut tx_context::TxContext) {}
}

module myso::tx_context {
    struct TxContext has drop {}
}
