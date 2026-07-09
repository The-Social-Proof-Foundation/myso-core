// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use async_trait::async_trait;

use crate::sources::http_client::HttpFetchClient;
use crate::sources::{
    ContentKind, DiscoveryDomain, DiscoverySource, RawDiscoveryRecord, SourceConfig, SourceHealth,
    SourceMetadata,
};

/// Real `DiscoverySource` that fetches RSS 2.0 / Atom feeds over HTTP and emits one
/// `RawDiscoveryRecord` per item with a verifiable `content_hash` of the feed body.
pub struct RssAdapter {
    client: HttpFetchClient,
}

impl RssAdapter {
    pub fn new() -> Self {
        Self {
            client: HttpFetchClient::new(),
        }
    }

    pub fn with_client(client: HttpFetchClient) -> Self {
        Self { client }
    }

    async fn fetch_feed_items(
        &self,
        feed_url: &str,
        trust_score: f64,
    ) -> anyhow::Result<Vec<RawDiscoveryRecord>> {
        let fetched = self.client.get_text(feed_url).await?;
        let items = parse_feed_items(&fetched.body, feed_url)?;
        let mut records = Vec::with_capacity(items.len());
        for item in items {
            records.push(RawDiscoveryRecord {
                external_source_url: item.link,
                media_type: "text/html".to_string(),
                content_kind: ContentKind::Text,
                title: Some(item.title),
                creator_x_handle: None,
                trust_score,
                content_hash: Some(fetched.content_hash.clone()),
                metadata: serde_json::json!({
                    "feed_url": feed_url,
                    "published_at": item.published_at,
                }),
            });
        }
        Ok(records)
    }
}

impl Default for RssAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DiscoverySource for RssAdapter {
    fn id(&self) -> &str {
        "rss"
    }

    fn domain(&self) -> DiscoveryDomain {
        DiscoveryDomain::Factual
    }

    fn supports(&self, config: &SourceConfig) -> bool {
        config.adapter_type == "rss"
            && config.enabled
            && !config.config.feed_urls.is_empty()
    }

    async fn discover(&self, config: &SourceConfig) -> anyhow::Result<Vec<RawDiscoveryRecord>> {
        let mut all = Vec::new();
        for feed_url in &config.config.feed_urls {
            match self.fetch_feed_items(feed_url, config.trust_score).await {
                Ok(mut records) => all.append(&mut records),
                Err(e) => {
                    tracing::warn!("rss adapter: feed {feed_url} failed: {e:#}");
                }
            }
        }
        Ok(all)
    }

    async fn health(&self) -> SourceHealth {
        SourceHealth {
            healthy: true,
            message: "rss adapter ready".into(),
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id().into(),
            description: "Real RSS/Atom feed discovery (live HTTP fetch)".into(),
            domain: DiscoveryDomain::Factual,
        }
    }
}

struct FeedItem {
    title: String,
    link: String,
    published_at: Option<String>,
}

/// Minimal tolerant parser covering RSS 2.0 (`<item>`/`<link>`/`<title>`/`<pubDate>`)
/// and Atom (`<entry>`/`<link href="...">`/`<title>`/`<updated>` or `<published>`).
/// Avoids pulling a full feed crate; real-world feeds are messy, so this is forgiving.
fn parse_feed_items(xml: &str, feed_url: &str) -> anyhow::Result<Vec<FeedItem>> {
    if xml.contains("<entry") {
        parse_atom(xml, feed_url)
    } else if xml.contains("<item") {
        parse_rss2(xml)
    } else {
        Err(anyhow!(
            "feed {feed_url} is neither RSS 2.0 nor Atom (no <item>/<entry> found)"
        ))
    }
}

fn parse_rss2(xml: &str) -> anyhow::Result<Vec<FeedItem>> {
    let mut items = Vec::new();
    for item_chunk in split_tags(xml, "item") {
        let title = extract_inner(&item_chunk, "title").unwrap_or_default().trim().to_string();
        let link = extract_inner(&item_chunk, "link")
            .unwrap_or_default()
            .trim()
            .to_string();
        let published_at = extract_inner(&item_chunk, "pubDate").map(|s| s.trim().to_string());
        if !link.is_empty() {
            items.push(FeedItem {
                title,
                link,
                published_at,
            });
        }
    }
    Ok(items)
}

fn parse_atom(xml: &str, feed_url: &str) -> anyhow::Result<Vec<FeedItem>> {
    let mut items = Vec::new();
    for entry_chunk in split_tags(xml, "entry") {
        let title = extract_inner(&entry_chunk, "title").unwrap_or_default().trim().to_string();
        let link = extract_link_href(&entry_chunk)
            .or_else(|| extract_inner(&entry_chunk, "link").map(|s| s.trim().to_string()))
            .unwrap_or_default();
        let published_at = extract_inner(&entry_chunk, "published")
            .or_else(|| extract_inner(&entry_chunk, "updated"))
            .map(|s| s.trim().to_string());
        if link.is_empty() {
            // Atom entries may carry id as a self link fallback.
            continue;
        }
        let absolute = resolve_url(feed_url, &link);
        items.push(FeedItem {
            title,
            link: absolute,
            published_at,
        });
    }
    Ok(items)
}

fn split_tags(xml: &str, tag: &str) -> Vec<String> {
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    let mut out = Vec::new();
    let mut rest = xml;
    while let Some(start) = rest.find(&open) {
        let after_open = &rest[start..];
        if let Some(end) = after_open.find(&close) {
            out.push(after_open[..end + close.len()].to_string());
            rest = &after_open[end + close.len()..];
        } else {
            break;
        }
    }
    out
}

fn extract_inner(chunk: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    let start = chunk.find(&open)?;
    let after_open = &chunk[start..];
    let gt = after_open.find('>')?;
    let body_start = after_open[gt + 1..].to_string();
    let body = &body_start[..body_start.find(&close)?];
    Some(decode_entities(body.trim()))
}

fn extract_link_href(chunk: &str) -> Option<String> {
    // Atom: <link href="..." rel="alternate"/> — prefer alternate; fall back to first href.
    let mut chosen: Option<String> = None;
    let mut first: Option<String> = None;
    let mut rest = chunk;
    while let Some(idx) = rest.find("<link") {
        let slice = &rest[idx..];
        let end = slice.find('>')?;
        let tag = &slice[..end + 1];
        let href = extract_attr(tag, "href");
        if let Some(href) = href {
            if first.is_none() {
                first = Some(href.clone());
            }
            if tag.contains("rel=\"alternate\"") || tag.contains("rel='alternate'") {
                chosen = Some(href);
                break;
            }
            if chosen.is_none() {
                chosen = Some(href);
            }
        }
        rest = &slice[end + 1..];
    }
    chosen.or(first)
}

fn extract_attr(tag: &str, attr: &str) -> Option<String> {
    let key = format!("{attr}=\"");
    if let Some(start) = tag.find(&key) {
        let after = &tag[start + key.len()..];
        let end = after.find('"')?;
        return Some(after[..end].to_string());
    }
    let key = format!("{attr}='");
    if let Some(start) = tag.find(&key) {
        let after = &tag[start + key.len()..];
        let end = after.find('\'')?;
        return Some(after[..end].to_string());
    }
    None
}

fn decode_entities(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
}

fn resolve_url(base: &str, link: &str) -> String {
    if link.starts_with("http://") || link.starts_with("https://") {
        return link.to_string();
    }
    if let Some(scheme_end) = base.find("://") {
        let host_start = &base[scheme_end + 3..];
        let host = host_start.split('/').next().unwrap_or("");
        if link.starts_with('/') {
            return format!("{}://{}{}", &base[..scheme_end], host, link);
        }
        return format!("{}://{}/{}", &base[..scheme_end], host, link);
    }
    link.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_rss2_items() {
        let xml = r#"<rss><channel>
            <title>Feed</title>
            <item><title>One</title><link>https://example.com/1</link><pubDate>Wed, 02 Oct 2024 13:00:00 GMT</pubDate></item>
            <item><title>Two</title><link>https://example.com/2</link></item>
        </channel></rss>"#;
        let items = parse_feed_items(xml, "https://example.com/feed").unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].title, "One");
        assert_eq!(items[0].link, "https://example.com/1");
        assert!(items[0].published_at.is_some());
    }

    #[test]
    fn parses_atom_entries_and_resolves_relative_links() {
        let xml = r#"<feed>
            <entry>
              <title>A</title>
              <link href="/posts/a" rel="alternate"/>
              <published>2024-10-02T13:00:00Z</published>
            </entry>
            <entry>
              <title>B</title>
              <link href="https://other.com/b" rel="alternate"/>
            </entry>
        </feed>"#;
        let items = parse_feed_items(xml, "https://blog.example.com/feed.xml").unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].link, "https://blog.example.com/posts/a");
        assert_eq!(items[1].link, "https://other.com/b");
    }
}
