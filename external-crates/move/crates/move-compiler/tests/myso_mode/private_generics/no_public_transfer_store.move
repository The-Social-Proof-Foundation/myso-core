// tests modules cannot use transfer internal functions outside of the defining module
// even if it has store

module a::m {
    use a::other;
    use myso::object::UID;
    use myso::transfer::{Self, Receiving};

    public fun t1(s: other::S) {
        transfer::transfer(s, @0x100);
    }

    public fun t2(s: other::S) {
        transfer::freeze_object(s);
    }

    public fun t3(s: other::S) {
        transfer::share_object(s);
    }

    public fun t4(p: &mut UID, s: Receiving<other::S>): other::S {
        transfer::receive(p, s)
    }

    public fun t6(s: other::S, p: myso::party::Party) {
        transfer::party_transfer(s, p);
    }
}

module a::other {
    struct S has key, store {
        id: myso::object::UID,
    }
}

module myso::object {
    struct UID has store {
        id: address,
    }
}

module myso::transfer {
    use myso::object::UID;

    struct Receiving<phantom T: key> {}

    public fun transfer<T: key>(_: T, _: address) {
        abort 0
    }

    public fun public_transfer<T: key + store>(_: T, _: address) {
        abort 0
    }

    public fun party_transfer<T: key>(_: T, _: myso::party::Party) {
        abort 0
    }

    public fun public_party_transfer<T: key + store>(_: T, _: myso::party::Party) {
        abort 0
    }

    public fun freeze_object<T: key>(_: T) {
        abort 0
    }

    public fun public_freeze_object<T: key + store>(_: T) {
        abort 0
    }

    public fun share_object<T: key>(_: T) {
        abort 0
    }

    public fun public_share_object<T: key + store>(_: T) {
        abort 0
    }

    public fun receive<T: key>(_: &mut UID, _: Receiving<T>): T {
        abort 0
    }

    public fun public_receive<T: key + store>(_: &mut UID, _: Receiving<T>): T {
        abort 0
    }
}

module myso::party {
    struct Party has copy, drop {}
}
