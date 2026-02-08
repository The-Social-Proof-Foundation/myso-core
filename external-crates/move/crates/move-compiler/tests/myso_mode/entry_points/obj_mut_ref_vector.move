// invalid, a mutable reference to vector of objects

module a::m {
    use myso::object;

    struct S has key { id: object::UID }

    public entry fun no(_: &mut vector<S>) {
        abort 0
    }
}

module myso::object {
    struct UID has store {
        id: address,
    }
}
