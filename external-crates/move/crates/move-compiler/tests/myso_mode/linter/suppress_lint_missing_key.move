module a::trigger_lint_cases {
    use myso::object::UID;

    // 4. Suppress warning
    #[allow(lint(missing_key))]
    struct SuppressWarning {
        id: UID,
    }
}

module myso::object {
    struct UID has store {
        id: address,
    }
}
