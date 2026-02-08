// valid, Clock by immutable reference

module a::m {
    public entry fun yes_clock_ref(_: &myso::clock::Clock) {
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
