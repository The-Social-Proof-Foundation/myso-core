module a::edge_cases {
    struct UID {}
    // Test case with a different UID type
    struct DifferentUID {
        id: myso::another::UID,
    }

    struct NotAnObject {
        id: UID,
    }
}

module myso::object {
    struct UID has store {
        id: address,
    }
}

module myso::another {
    struct UID has store {
        id: address,
    }
}
