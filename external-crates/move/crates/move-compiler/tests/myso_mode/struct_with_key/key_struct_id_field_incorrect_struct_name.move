// invalid, objects need UID not ID
module a::m {
    use myso::object;

    struct S has key {
        id: object::ID,
    }
}

module myso::object {
    struct ID has store {
        id: address,
    }
}
