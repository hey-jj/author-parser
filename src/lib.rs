//! Parse an npm "person" string into its parts.
//!
//! npm lets the `author`, `contributors`, and `maintainers` fields of a
//! `package.json` use a shorthand string of the form `Name <email> (url)`.
//! This crate parses that shorthand into an [`Author`] with optional `name`,
//! `email`, and `url`. Every part is optional and the parts may appear in any
//! order.
//!
//! # Examples
//!
//! ```
//! use parse_author_rs::{parse, Author};
//!
//! let a = parse("Jon Schlinkert <jon@example.com> (https://example.com)");
//! assert_eq!(a.name.as_deref(), Some("Jon Schlinkert"));
//! assert_eq!(a.email.as_deref(), Some("jon@example.com"));
//! assert_eq!(a.url.as_deref(), Some("https://example.com"));
//! ```
//!
//! A name-only string sets only `name`:
//!
//! ```
//! use parse_author_rs::parse;
//!
//! assert_eq!(parse("Jon Schlinkert").name.as_deref(), Some("Jon Schlinkert"));
//! ```
//!
//! Input with no parts returns the default (all `None`):
//!
//! ```
//! use parse_author_rs::{parse, Author};
//!
//! assert_eq!(parse(""), Author::default());
//! assert_eq!(parse("   "), Author::default());
//! ```
//!
//! # Grammar notes
//!
//! - A bracket token is `<...>` for email or `(...)` for url. The leading
//!   bracket decides the field, so mismatched pairs like `<x)` still read as
//!   email and `(x>` still read as url.
//! - Empty bracket tokens (`<>` or `()`) are dropped.
//! - At most two bracket tokens are allowed. Three or more fail to match and
//!   return the default.
//! - When two tokens set the same field, the second wins.
//! - The presence gate uses an ASCII word character check (`[A-Za-z0-9_]`).
//!   A string with no ASCII word character returns the default even if it
//!   holds non-ASCII letters. A name that does carry an ASCII word character
//!   keeps its full text, so `"Jön Müller"` parses to that name.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::sync::LazyLock;

use regex::Regex;

/// Tokenizes a person string into name and up to two bracket tokens.
///
/// Group layout on a match:
/// - group 1: name, the lazy run of text before the first `<` or `(`.
/// - group 2 / 3: first bracket token with its brackets / its inner value.
/// - group 4 / 5: second bracket token with its brackets / its inner value.
///
/// The trailing anchor plus the fixed shape mean a third bracket token makes
/// the whole pattern fail, which is the intended cap of two tokens.
static AUTHOR_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*([^<(]*?)\s*([<(]([^>)]*?)[>)])?\s*([<(]([^>)]*?)[>)])*\s*$")
        .expect("author pattern is valid")
});

/// A parsed person string.
///
/// Each field is `Some` only when the corresponding part was found and
/// non-empty. An input with no usable parts yields [`Author::default`], where
/// every field is `None`.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Author {
    /// The name segment, trimmed of surrounding whitespace. Internal
    /// whitespace is kept verbatim.
    pub name: Option<String>,
    /// The email read from a `<...>` token, without the brackets.
    pub email: Option<String>,
    /// The url read from a `(...)` token, without the brackets.
    pub url: Option<String>,
}

/// Parse a person string into an [`Author`].
///
/// Returns the default ([`Author::default`]) when `input` is empty, holds no
/// ASCII word character, or does not fit the grammar.
///
/// # Examples
///
/// ```
/// use parse_author_rs::parse;
///
/// let a = parse("Sean Lang <slang800@gmail.com> (http://slang.cx)");
/// assert_eq!(a.name.as_deref(), Some("Sean Lang"));
/// assert_eq!(a.email.as_deref(), Some("slang800@gmail.com"));
/// assert_eq!(a.url.as_deref(), Some("http://slang.cx"));
/// ```
///
/// Fields may appear in any order:
///
/// ```
/// use parse_author_rs::parse;
///
/// let a = parse("(https://example.com) <jon@example.com>");
/// assert_eq!(a.email.as_deref(), Some("jon@example.com"));
/// assert_eq!(a.url.as_deref(), Some("https://example.com"));
/// assert_eq!(a.name, None);
/// ```
pub fn parse(input: &str) -> Author {
    // Presence gate. Skip the match when there is nothing word-like to keep.
    // ASCII word characters only, to match the npm grammar's gate exactly.
    if input.is_empty() || !input.bytes().any(is_ascii_word) {
        return Author::default();
    }

    let caps = match AUTHOR_RE.captures(input) {
        Some(caps) => caps,
        None => return Author::default(),
    };

    let mut author = Author::default();

    // Name comes from group 1. An empty capture means no name.
    if let Some(name) = caps.get(1).map(|m| m.as_str()).filter(|s| !s.is_empty()) {
        author.name = Some(name.to_string());
    }

    // Each bracket token is a (whole-token, inner-value) pair. The first
    // character of the whole token picks the field.
    for (token_group, value_group) in [(2, 3), (4, 5)] {
        let token = caps.get(token_group).map(|m| m.as_str()).unwrap_or("");
        let value = caps.get(value_group).map(|m| m.as_str()).unwrap_or("");
        if token.is_empty() || value.is_empty() {
            continue;
        }
        match token.as_bytes()[0] {
            b'<' => author.email = Some(value.to_string()),
            b'(' => author.url = Some(value.to_string()),
            _ => {}
        }
    }

    author
}

/// True for an ASCII word byte: a letter, a digit, or an underscore.
fn is_ascii_word(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}
