# parse-author-rs

Parse an npm "person" string into its parts.

npm lets the `author`, `contributors`, and `maintainers` fields of a
`package.json` use a shorthand string of the form `Name <email> (url)`. This
crate parses that shorthand into an `Author` with optional `name`, `email`, and
`url`. Every part is optional and the parts may appear in any order.

## Installation

```toml
[dependencies]
parse-author-rs = "0.1"
```

## Usage

```rust
use parse_author_rs::parse;

let a = parse("Jon Schlinkert <jon@example.com> (https://example.com)");
assert_eq!(a.name.as_deref(), Some("Jon Schlinkert"));
assert_eq!(a.email.as_deref(), Some("jon@example.com"));
assert_eq!(a.url.as_deref(), Some("https://example.com"));
```

A field is `Some` only when the matching part is present and non-empty. An
input with no usable parts returns the default, where every field is `None`.

```rust
use parse_author_rs::{parse, Author};

assert_eq!(parse(""), Author::default());
assert_eq!(parse("   "), Author::default());
```

## Supported formats

Any subset of the parts works, in any order:

```text
Name
Name <email> (url)
Name <email>(url)
Name<email> (url)
Name<email>(url)
Name (url) <email>
Name (url)<email>
Name(url) <email>
Name(url)<email>
Name (url)
Name(url)
Name <email>
Name<email>
<email> (url)
<email>(url)
(url) <email>
(url)<email>
<email>
(url)
```

## Behavior notes

- A bracket token is `<...>` for email or `(...)` for url. The leading bracket
  decides the field, so mismatched pairs like `<x)` still read as email.
- Empty bracket tokens (`<>` or `()`) are dropped.
- At most two bracket tokens are allowed. Three or more fail and return the
  default.
- When two tokens set the same field, the second wins.
- The presence gate uses an ASCII word character check. A string with no ASCII
  word character returns the default, even when it holds non-ASCII letters. A
  name that carries an ASCII word character keeps its full text.

## License

Licensed under the [MIT license](LICENSE).
