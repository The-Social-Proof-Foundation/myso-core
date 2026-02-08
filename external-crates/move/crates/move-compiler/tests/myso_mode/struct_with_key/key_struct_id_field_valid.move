// valid
module a::m {
    use myso::object;

    struct S has key {
        id: object::UID,
    }
}

module myso::object {
    struct UID has store {
        id: address,
    }
}
