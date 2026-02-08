// invalid, Clock by value

module a::m {
    public entry fun no_clock_val(_: myso::clock::Clock) {
        abort 0
    }
}

module myso::clock {
    struct Clock has key {
        id: myso::object::UID,
    }
}

module myso::object {
    struct UID has store {
        id: address,
    }
}
