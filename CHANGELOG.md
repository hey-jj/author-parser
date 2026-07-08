# Changelog

## [0.1.1] - 2026-07-07

### Fixed
- Kept the published library target name as `author_parser` through Cargo's package-name default. (#14)

### Documentation
- Corrected the adjacent bracket token notes for mixed runs such as `<a>(u)<b>`, where only the first bracket token and final trailing token affect the result. (#13)
