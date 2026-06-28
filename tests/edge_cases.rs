//! Edge-case behavior the canonical suite does not exercise.
//!
//! These pin grammar quirks: the ASCII word gate, last-token-wins, the
//! trailing-anchor failure, mismatched brackets, and inner whitespace.

use parse_author_rs::{parse, Author};

fn author(name: Option<&str>, email: Option<&str>, url: Option<&str>) -> Author {
    Author {
        name: name.map(Into::into),
        email: email.map(Into::into),
        url: url.map(Into::into),
    }
}

#[test]
fn whitespace_only_returns_default() {
    assert_eq!(parse("   "), Author::default());
    assert_eq!(parse("\t"), Author::default());
    assert_eq!(parse("\n"), Author::default());
}

#[test]
fn digits_parse_as_name() {
    assert_eq!(parse("123"), author(Some("123"), None, None));
}

#[test]
fn underscore_parses_as_name() {
    assert_eq!(parse("_"), author(Some("_"), None, None));
}

#[test]
fn inner_whitespace_in_name_is_preserved() {
    assert_eq!(
        parse("Jon\tSchlinkert"),
        author(Some("Jon\tSchlinkert"), None, None)
    );
    assert_eq!(
        parse("Jon   Q   Public"),
        author(Some("Jon   Q   Public"), None, None)
    );
}

#[test]
fn non_ascii_only_name_is_dropped() {
    // No ASCII word character, so the gate returns the default.
    assert_eq!(parse("é"), Author::default());
}

#[test]
fn name_with_ascii_word_keeps_full_unicode_text() {
    assert_eq!(parse("Jön Müller"), author(Some("Jön Müller"), None, None));
}

#[test]
fn duplicate_url_last_wins() {
    assert_eq!(parse("(url) (url2)"), author(None, None, Some("url2")));
}

#[test]
fn duplicate_email_last_wins() {
    assert_eq!(
        parse("<a@b.com> <c@d.com>"),
        author(None, Some("c@d.com"), None)
    );
}

#[test]
fn trailing_junk_returns_default() {
    assert_eq!(
        parse("name with (paren in middle) trailing"),
        Author::default()
    );
}

#[test]
fn unclosed_brackets_return_default() {
    assert_eq!(parse("a <b"), Author::default());
    assert_eq!(parse("a (b"), Author::default());
    assert_eq!(parse("Na<me"), Author::default());
    assert_eq!(parse("Jon (foo"), Author::default());
}

#[test]
fn trailing_bracket_junk_returns_default() {
    assert_eq!(parse("<a>b>"), Author::default());
    assert_eq!(parse("(a)b)"), Author::default());
}

#[test]
fn three_or_more_bracket_groups_return_default() {
    assert_eq!(parse("Name <a@b.com> (u1) (u2)"), Author::default());
    assert_eq!(parse("<e> (u1) (u2)"), Author::default());
}

#[test]
fn mismatched_brackets_classify_by_opening() {
    // Opened with `<`, closed with `)`: still email.
    assert_eq!(
        parse("Name <jon@x.com)"),
        author(Some("Name"), Some("jon@x.com"), None)
    );
    // Opened with `(`, closed with `>`: still url.
    assert_eq!(parse("Name (url>"), author(Some("Name"), None, Some("url")));
}

#[test]
fn minimal_tokens_need_no_scheme_or_at_sign() {
    assert_eq!(parse("a<b>"), author(Some("a"), Some("b"), None));
}

#[test]
fn order_independent_without_whitespace() {
    assert_eq!(
        parse("<e@x.com>(u)"),
        author(None, Some("e@x.com"), Some("u"))
    );
    assert_eq!(
        parse("(u)<e@x.com>"),
        author(None, Some("e@x.com"), Some("u"))
    );
    assert_eq!(
        parse("Jon<e@x>(u)"),
        author(Some("Jon"), Some("e@x"), Some("u"))
    );
}

#[test]
fn url_first_misorder_no_whitespace() {
    let expected = author(Some("Name"), Some("email"), Some("url"));
    assert_eq!(parse("Name(url)<email>"), expected);
    assert_eq!(parse("Name (url)<email>"), expected);
    assert_eq!(parse("Name(url) <email>"), expected);
}

#[test]
fn supported_formats_from_docs() {
    // Each advertised form parses without panic and keeps the named parts.
    let name = Some("Name");
    assert_eq!(parse("Name"), author(name, None, None));
    assert_eq!(
        parse("Name <email> (url)"),
        author(name, Some("email"), Some("url"))
    );
    assert_eq!(
        parse("Name <email>(url)"),
        author(name, Some("email"), Some("url"))
    );
    assert_eq!(
        parse("Name<email> (url)"),
        author(name, Some("email"), Some("url"))
    );
    assert_eq!(
        parse("Name<email>(url)"),
        author(name, Some("email"), Some("url"))
    );
    assert_eq!(
        parse("Name (url) <email>"),
        author(name, Some("email"), Some("url"))
    );
    assert_eq!(
        parse("Name (url)<email>"),
        author(name, Some("email"), Some("url"))
    );
    assert_eq!(
        parse("Name(url) <email>"),
        author(name, Some("email"), Some("url"))
    );
    assert_eq!(
        parse("Name(url)<email>"),
        author(name, Some("email"), Some("url"))
    );
    assert_eq!(parse("Name (url)"), author(name, None, Some("url")));
    assert_eq!(parse("Name(url)"), author(name, None, Some("url")));
    assert_eq!(parse("Name <email>"), author(name, Some("email"), None));
    assert_eq!(parse("Name<email>"), author(name, Some("email"), None));
    assert_eq!(
        parse("<email> (url)"),
        author(None, Some("email"), Some("url"))
    );
    assert_eq!(
        parse("<email>(url)"),
        author(None, Some("email"), Some("url"))
    );
    assert_eq!(
        parse("(url) <email>"),
        author(None, Some("email"), Some("url"))
    );
    assert_eq!(
        parse("(url)<email>"),
        author(None, Some("email"), Some("url"))
    );
    assert_eq!(parse("<email>"), author(None, Some("email"), None));
    assert_eq!(parse("(url)"), author(None, None, Some("url")));
}
