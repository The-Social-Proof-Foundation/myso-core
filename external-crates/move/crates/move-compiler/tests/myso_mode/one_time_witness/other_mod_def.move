// invalid, one-time witness type candidate used in a different module

module a::n {
    use myso::myso;
    use myso::tx_context;

    fun init(_otw: myso::MYSO, _ctx: &mut tx_context::TxContext) {}
}

module myso::tx_context {
    struct TxContext has drop {}
}

module myso::myso {
    struct MYSO has drop {}
}
