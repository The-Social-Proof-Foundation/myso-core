// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Per-address deposit completion callbacks (platform-neutral).

use std::net::IpAddr;
use url::Url;

/// Validate a platform callback URL. HTTPS only; reject loopback / private hosts.
/// When `allowlist` is non-empty, the host must match one of those names.
pub fn validate_deposit_callback_url(
    raw: &str,
    allowlist: &[String],
) -> Result<String, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("callbackUrl is empty".to_string());
    }

    let url = Url::parse(trimmed).map_err(|e| format!("Invalid callbackUrl: {e}"))?;
    if url.scheme() != "https" {
        return Err("callbackUrl must use https".to_string());
    }

    let host = url
        .host_str()
        .ok_or_else(|| "callbackUrl is missing a host".to_string())?;

    if host.eq_ignore_ascii_case("localhost") || host.ends_with(".localhost") {
        return Err("callbackUrl host is not allowed".to_string());
    }

    if let Ok(ip) = host.parse::<IpAddr>() {
        if is_blocked_ip(ip) {
            return Err("callbackUrl host is not allowed".to_string());
        }
    }

    if !allowlist.is_empty()
        && !allowlist
            .iter()
            .any(|allowed| allowed.eq_ignore_ascii_case(host))
    {
        return Err("callbackUrl host is not allowed".to_string());
    }

    Ok(url.to_string())
}

pub fn parse_callback_host_allowlist(raw: Option<&str>) -> Vec<String> {
    raw.unwrap_or("")
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn is_blocked_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            v4.is_loopback()
                || v4.is_private()
                || v4.is_link_local()
                || v4.is_unspecified()
                || v4.octets()[0] == 0
        }
        IpAddr::V6(v6) => v6.is_loopback() || v6.is_unspecified() || v6.is_unique_local(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_http_and_localhost() {
        assert!(validate_deposit_callback_url("http://example.com/cb", &[]).is_err());
        assert!(validate_deposit_callback_url("https://localhost/cb", &[]).is_err());
        assert!(validate_deposit_callback_url("https://127.0.0.1/cb", &[]).is_err());
        assert!(validate_deposit_callback_url("https://10.0.0.4/cb", &[]).is_err());
    }

    #[test]
    fn accepts_https_public_host() {
        let url = validate_deposit_callback_url(
            "https://api.example.com/offramp/internal/bridge-complete",
            &[],
        )
        .unwrap();
        assert_eq!(
            url,
            "https://api.example.com/offramp/internal/bridge-complete"
        );
    }

    #[test]
    fn allowlist_restricts_host() {
        let allow = vec!["api.example.com".to_string()];
        assert!(
            validate_deposit_callback_url("https://api.example.com/cb", &allow).is_ok()
        );
        assert!(
            validate_deposit_callback_url("https://evil.example/cb", &allow).is_err()
        );
    }
}
