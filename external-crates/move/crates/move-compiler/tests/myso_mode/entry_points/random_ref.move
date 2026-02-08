// valid Random by immutable reference

module a::m {
    public entry fun yes_random_ref(_: &myso::random::Random) {
        abort 0
    }
}

module myso::random {
    struct Random has key {
        id: myso::object::UID,
    }
}

module myso::object {
    struct UID has store {
        id: address,
    }
}
