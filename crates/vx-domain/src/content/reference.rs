use std::net::{Ipv4Addr, Ipv6Addr};

use crate::shared::{FieldError, License, Validation};

/// Study reference kinds supported before the F2 library.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceType {
    /// Scholarly paper.
    Paper,
    /// A specific book edition.
    Book,
    /// Course or personal notes.
    Notes,
    /// Linked video.
    Video,
    /// Linked web page.
    Webpage,
}

/// Study metadata only; no downloaded files or F2 material identifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    /// Kind of referenced resource.
    pub kind: ReferenceType,
    /// Resource title.
    pub title: String,
    /// Credited resource authors.
    pub authors: Vec<String>,
    /// Optional HTTP(S) link, without credentials.
    pub url: Option<String>,
    /// Optional digital object identifier.
    pub doi: Option<String>,
    /// Optional arXiv identifier.
    pub arxiv: Option<String>,
    /// Optional ISBN identifier.
    pub isbn: Option<String>,
    /// Optional human-readable section, page or timestamp locator.
    pub locator: Option<String>,
    /// Rights of the linked edition; restrictive licenses are allowed for links.
    pub license: License,
}

impl Reference {
    /// Requires metadata and one locator identifier, and rejects unsafe or malformed links.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.text(&self.title, "title");
        validation.texts(&self.authors, "authors");
        validation.nested(self.license.validate(), "license");
        let identifiers = [
            (&self.url, "url"),
            (&self.doi, "doi"),
            (&self.arxiv, "arxiv"),
            (&self.isbn, "isbn"),
        ];
        validation.require(
            identifiers
                .iter()
                .any(|(value, _)| value.as_ref().is_some_and(|value| !value.trim().is_empty())),
            "url",
            "At least one of url, doi, arxiv or isbn is required",
        );
        for (value, field) in identifiers {
            validation.optional_text(value, field);
        }
        validation.optional_text(&self.locator, "locator");
        if let Some(url) = &self.url {
            validation.require(
                valid_http_url(url),
                "url",
                "Must be an HTTP(S) URL with a valid host and no credentials or control characters",
            );
        }
        validation.finish()
    }
}

// This intentionally accepts a conservative HTTP(S) subset, without network access or URL crates.
fn valid_http_url(url: &str) -> bool {
    if url
        .chars()
        .any(|character| character.is_control() || character.is_whitespace() || character == '\\')
        || !valid_percent_escapes(url)
    {
        return false;
    }
    let Some((scheme, rest)) = url.split_once("://") else {
        return false;
    };
    if !scheme.eq_ignore_ascii_case("http") && !scheme.eq_ignore_ascii_case("https") {
        return false;
    }
    let authority = rest.split(['/', '?', '#']).next().unwrap_or_default();
    if authority.is_empty() || authority.contains('@') {
        return false;
    }
    valid_authority(authority)
}

fn valid_percent_escapes(value: &str) -> bool {
    let mut bytes = value.bytes();
    while let Some(byte) = bytes.next() {
        if byte != b'%' {
            continue;
        }
        let (Some(high), Some(low)) = (bytes.next(), bytes.next()) else {
            return false;
        };
        let (Some(high), Some(low)) = ((high as char).to_digit(16), (low as char).to_digit(16))
        else {
            return false;
        };
        let decoded = high * 16 + low;
        if decoded < 0x20 || decoded == 0x7f {
            return false;
        }
    }
    true
}

fn valid_authority(authority: &str) -> bool {
    if let Some(ipv6) = authority.strip_prefix('[') {
        let Some((address, suffix)) = ipv6.split_once(']') else {
            return false;
        };
        return address.parse::<Ipv6Addr>().is_ok()
            && (suffix.is_empty() || suffix.strip_prefix(':').is_some_and(valid_port));
    }
    let (host, port) = authority
        .split_once(':')
        .map_or((authority, None), |(host, port)| (host, Some(port)));
    valid_host(host) && port.is_none_or(valid_port)
}

fn valid_port(port: &str) -> bool {
    !port.is_empty()
        && port.bytes().all(|byte| byte.is_ascii_digit())
        && port.parse::<u16>().is_ok()
}

fn valid_host(host: &str) -> bool {
    if host.is_empty() || host.len() > 253 {
        return false;
    }
    if host
        .bytes()
        .all(|byte| byte.is_ascii_digit() || byte == b'.')
    {
        return host.parse::<Ipv4Addr>().is_ok();
    }
    host.strip_suffix('.')
        .unwrap_or(host)
        .split('.')
        .all(|label| {
            !label.is_empty()
                && label.len() <= 63
                && !label.starts_with('-')
                && !label.ends_with('-')
                && label
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        })
}

#[cfg(test)]
mod tests {
    use super::valid_http_url;

    #[test]
    fn permits_only_safe_well_formed_http_links() {
        for url in [
            "https://example.org/chapter?q=1#p2",
            "http://localhost:8080/a",
            "HTTPS://example.org/",
            "https://[::1]:443/a",
            "https://127.0.0.1/a",
            "https://example.org/a%2Fb",
            "https://example.org/a%20chapter",
        ] {
            assert!(valid_http_url(url), "{url}");
        }
        for url in [
            "javascript:alert(1)",
            "data:text/html,hi",
            "file:///tmp/a",
            "ftp://example.org",
            "//example.org",
            "https:///chapter",
            "https://?x",
            "https://user:pass@example.org",
            "https://user@example.org",
            "https://example.org\\@evil.org",
            "https://exa mple.org",
            "https://example.org/\n",
            "https://example.org/%0a",
            "https://example.org/%7F",
            "https://example.org/%",
            "https://example.org/%zz",
            "https://example.org:65536",
            "https://example.org:",
            "https://[bad]/",
            "https://::1/",
            "https://.example.org",
            "https://-bad.org",
            "https://a..org",
            "https://%65xample.org",
            "https://999.0.0.1",
        ] {
            assert!(!valid_http_url(url), "{url}");
        }
    }
}
