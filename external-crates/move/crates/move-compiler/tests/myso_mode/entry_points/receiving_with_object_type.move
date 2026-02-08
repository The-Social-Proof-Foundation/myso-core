// valid, Receiving type with object type param

module a::m {
    use myso::object;
    use myso::transfer::Receiving;

    struct S has key { id: object::UID }

    public entry fun yes(_: Receiving<S>) {}
}

module myso::object {
    struct UID has store {
        id: address,
    }
}

module myso::transfer {
    struct Receiving<phantom T: key> has drop {
        id: address,
    }
}
