// invalid Random by value

module a::m {
    public entry fun no_random_val(_: myso::random::Random) {
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
