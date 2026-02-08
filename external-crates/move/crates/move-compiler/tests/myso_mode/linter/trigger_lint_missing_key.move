module a::trigger_lint_cases {
    use myso::object::UID;

    // This should trigger the linter warning (true positive)
    struct MissingKeyAbility {
        id: UID,
    }
}

module myso::object {
    struct UID has store {
        id: address,
    }
}
