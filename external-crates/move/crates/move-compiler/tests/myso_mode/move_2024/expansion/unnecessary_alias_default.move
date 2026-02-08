module myso::object {
    public struct ID()
    public struct UID()
}
module myso::transfer {

}
module myso::tx_context {
    public struct TxContext()
}

module a::m {
    use myso::object::{Self, ID, UID};
    use myso::transfer;
    use myso::tx_context::{Self, TxContext};
}
