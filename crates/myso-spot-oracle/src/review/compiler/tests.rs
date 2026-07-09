// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use crate::review::compiler::fixtures;
    use crate::review::compiler::{test_registry, ResolverCompiler};
    use crate::types::ResolverKind;

    #[test]
    fn all_resolver_kinds_compile() {
        let registry = test_registry();
        for (name, claim) in [
            ("price", fixtures::btc_price_claim()),
            ("release", fixtures::github_release_claim()),
            ("event", fixtures::rss_event_claim()),
            ("custom_http", fixtures::custom_http_claim()),
        ] {
            let compiled = ResolverCompiler::compile(&claim, &registry, &[])
                .unwrap_or_else(|e| panic!("{name} compile failed: {e}"));
            assert!(!compiled.compile_fingerprint.is_empty());
            assert!(compiled.resolver_definition.source_ids.len() >= 1);
        }
    }

    #[test]
    fn unsupported_category_fails_compile() {
        let registry = test_registry();
        let err = ResolverCompiler::compile(&fixtures::unsupported_claim(), &registry, &[]);
        assert!(err.is_err());
    }

    #[test]
    fn price_kind_matches() {
        let registry = test_registry();
        let compiled =
            ResolverCompiler::compile(&fixtures::btc_price_claim(), &registry, &[]).unwrap();
        assert_eq!(
            compiled.resolver_definition.resolver_kind,
            ResolverKind::PriceThreshold
        );
    }
}
