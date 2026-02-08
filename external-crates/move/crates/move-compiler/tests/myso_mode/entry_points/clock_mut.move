// invalid, Clock by mutable reference

module a::m {
    public entry fun no_clock_mut(_: &mut myso::clock::Clock) {
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
