//! Behavioral parity tests.
//!
//! Each test mirrors one assertion from the canonical npm test suite. The
//! grouping matches that suite so coverage maps one to one.

use parse_author_rs::{parse, Author};

/// Build an expected [`Author`] from optional parts.
fn author(name: Option<&str>, email: Option<&str>, url: Option<&str>) -> Author {
    Author {
        name: name.map(Into::into),
        email: email.map(Into::into),
        url: url.map(Into::into),
    }
}

mod empty {
    use super::*;

    #[test]
    fn empty_string_returns_default() {
        assert_eq!(parse(""), author(None, None, None));
    }

    #[test]
    fn empty_url_placeholders() {
        assert_eq!(parse(" ()"), author(None, None, None));
        assert_eq!(
            parse("Jon Schlinkert ()"),
            author(Some("Jon Schlinkert"), None, None)
        );
        assert_eq!(
            parse("Jon Schlinkert <jon.schlinkert@sellside.com> ()"),
            author(
                Some("Jon Schlinkert"),
                Some("jon.schlinkert@sellside.com"),
                None
            )
        );
        assert_eq!(
            parse("<jon.schlinkert@sellside.com> ()"),
            author(None, Some("jon.schlinkert@sellside.com"), None)
        );
    }

    #[test]
    fn empty_email_placeholders() {
        assert_eq!(parse("<>"), author(None, None, None));
        assert_eq!(
            parse("Jon Schlinkert <>"),
            author(Some("Jon Schlinkert"), None, None)
        );
        assert_eq!(
            parse("<> (https://github.com/jonschlinkert)"),
            author(None, None, Some("https://github.com/jonschlinkert"))
        );
    }

    #[test]
    fn empty_email_and_url_placeholders() {
        assert_eq!(parse("<> ()"), author(None, None, None));
    }
}

mod name {
    use super::*;

    #[test]
    fn name_only() {
        assert_eq!(
            parse("Jon Schlinkert"),
            author(Some("Jon Schlinkert"), None, None)
        );
    }

    #[test]
    fn name_with_surrounding_whitespace() {
        let expected = author(Some("Jon Schlinkert"), None, None);
        assert_eq!(parse(" Jon Schlinkert"), expected);
        assert_eq!(parse("Jon Schlinkert "), expected);
        assert_eq!(parse(" Jon Schlinkert "), expected);
    }
}

mod email {
    use super::*;

    #[test]
    fn email_only() {
        assert_eq!(
            parse("<jon.schlinkert@sellside.com>"),
            author(None, Some("jon.schlinkert@sellside.com"), None)
        );
    }

    #[test]
    fn email_with_surrounding_whitespace() {
        let expected = author(None, Some("jon.schlinkert@sellside.com"), None);
        assert_eq!(parse(" <jon.schlinkert@sellside.com>"), expected);
        assert_eq!(parse("<jon.schlinkert@sellside.com> "), expected);
        assert_eq!(parse(" <jon.schlinkert@sellside.com> "), expected);
    }
}

mod url {
    use super::*;

    #[test]
    fn url_only() {
        assert_eq!(
            parse("(https://github.com/jonschlinkert)"),
            author(None, None, Some("https://github.com/jonschlinkert"))
        );
    }

    #[test]
    fn url_with_surrounding_whitespace() {
        let expected = author(None, None, Some("https://github.com/jonschlinkert"));
        assert_eq!(parse(" (https://github.com/jonschlinkert)"), expected);
        assert_eq!(parse("(https://github.com/jonschlinkert) "), expected);
        assert_eq!(parse(" (https://github.com/jonschlinkert) "), expected);
    }
}

mod name_and_url {
    use super::*;

    #[test]
    fn name_and_url() {
        assert_eq!(
            parse("Jon Schlinkert (https://github.com/jonschlinkert)"),
            author(
                Some("Jon Schlinkert"),
                None,
                Some("https://github.com/jonschlinkert")
            )
        );
    }
}

mod name_and_email {
    use super::*;

    fn expected() -> Author {
        author(
            Some("Jon Schlinkert"),
            Some("jon.schlinkert@sellside.com"),
            None,
        )
    }

    #[test]
    fn no_extra_whitespace() {
        assert_eq!(
            parse("Jon Schlinkert <jon.schlinkert@sellside.com>"),
            expected()
        );
    }

    #[test]
    fn leading_whitespace() {
        assert_eq!(
            parse(" Jon Schlinkert <jon.schlinkert@sellside.com>"),
            expected()
        );
    }

    #[test]
    fn trailing_whitespace() {
        assert_eq!(
            parse("Jon Schlinkert <jon.schlinkert@sellside.com> "),
            expected()
        );
    }

    #[test]
    fn leading_and_trailing_whitespace() {
        assert_eq!(
            parse(" Jon Schlinkert <jon.schlinkert@sellside.com> "),
            expected()
        );
    }
}

mod name_email_and_url {
    use super::*;

    fn expected() -> Author {
        author(
            Some("Jon Schlinkert"),
            Some("jon.schlinkert@sellside.com"),
            Some("https://github.com/jonschlinkert"),
        )
    }

    #[test]
    fn spaces_between_all() {
        assert_eq!(
            parse(
                "Jon Schlinkert <jon.schlinkert@sellside.com> (https://github.com/jonschlinkert)"
            ),
            expected()
        );
    }

    #[test]
    fn no_space_before_email() {
        assert_eq!(
            parse("Jon Schlinkert<jon.schlinkert@sellside.com> (https://github.com/jonschlinkert)"),
            expected()
        );
    }

    #[test]
    fn no_space_before_url() {
        assert_eq!(
            parse("Jon Schlinkert <jon.schlinkert@sellside.com>(https://github.com/jonschlinkert)"),
            expected()
        );
    }

    #[test]
    fn no_separating_whitespace() {
        assert_eq!(
            parse("Jon Schlinkert<jon.schlinkert@sellside.com>(https://github.com/jonschlinkert)"),
            expected()
        );
    }
}

mod email_and_url {
    use super::*;

    fn expected() -> Author {
        author(
            None,
            Some("jon.schlinkert@sellside.com"),
            Some("https://github.com/jonschlinkert"),
        )
    }

    #[test]
    fn no_extra_whitespace() {
        assert_eq!(
            parse("<jon.schlinkert@sellside.com> (https://github.com/jonschlinkert)"),
            expected()
        );
    }

    #[test]
    fn leading_whitespace() {
        assert_eq!(
            parse(" <jon.schlinkert@sellside.com> (https://github.com/jonschlinkert)"),
            expected()
        );
    }

    #[test]
    fn trailing_whitespace() {
        assert_eq!(
            parse("<jon.schlinkert@sellside.com> (https://github.com/jonschlinkert) "),
            expected()
        );
    }

    #[test]
    fn leading_and_trailing_whitespace() {
        assert_eq!(
            parse(" <jon.schlinkert@sellside.com> (https://github.com/jonschlinkert) "),
            expected()
        );
    }

    #[test]
    fn no_separating_whitespace() {
        assert_eq!(
            parse("<jon.schlinkert@sellside.com>(https://github.com/jonschlinkert)"),
            expected()
        );
    }
}

mod misordered {
    use super::*;

    #[test]
    fn url_before_email_with_name() {
        assert_eq!(
            parse(
                "Jon Schlinkert (https://github.com/jonschlinkert) <jon.schlinkert@sellside.com>"
            ),
            author(
                Some("Jon Schlinkert"),
                Some("jon.schlinkert@sellside.com"),
                Some("https://github.com/jonschlinkert")
            )
        );
    }

    #[test]
    fn url_before_email_no_name() {
        assert_eq!(
            parse("(https://github.com/jonschlinkert) <jon.schlinkert@sellside.com>"),
            author(
                None,
                Some("jon.schlinkert@sellside.com"),
                Some("https://github.com/jonschlinkert")
            )
        );
    }
}
