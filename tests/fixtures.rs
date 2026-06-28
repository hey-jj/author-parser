//! Data-driven runner over the shared golden table.
//!
//! Each row in `fixtures/cases.json` holds an input and the expected name,
//! email, and url (null when absent). One source of truth keeps the parity
//! and edge-case checks in sync.

use parse_author_rs::{parse, Author};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Case {
    input: String,
    name: Option<String>,
    email: Option<String>,
    url: Option<String>,
}

#[test]
fn golden_cases_match() {
    let raw = include_str!("fixtures/cases.json");
    let cases: Vec<Case> = serde_json::from_str(raw).expect("cases.json parses");
    assert!(!cases.is_empty(), "golden table is empty");

    for case in cases {
        let expected = Author {
            name: case.name.clone(),
            email: case.email.clone(),
            url: case.url.clone(),
        };
        let actual = parse(&case.input);
        assert_eq!(
            actual, expected,
            "input {:?} parsed to {:?}, expected {:?}",
            case.input, actual, expected
        );
    }
}
