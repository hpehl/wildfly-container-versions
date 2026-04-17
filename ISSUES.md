# Code Review Issues

## HIGH

- [x] **H1. Regex compiled on every call** — Fixed: moved regexes to `lazy_static` statics (`VERSION_RE`, `VERSIONS_RE`).
- [x] **H2. Port arithmetic can overflow** — Fixed: `http_port()` and `management_port()` now use `checked_add` with panic on overflow.

## MEDIUM

- [x] **M1. `format!` inside `bail!` is redundant** — Fixed: removed redundant `format!()` from all 8 `bail!()` calls.
- [ ] **M2. Mutation in `VERSIONS` init** — `lib.rs:25-62` — Standard `lazy_static` pattern, acceptable. Consider `phf` if the map grows significantly.
- [ ] **M3. `versions()` regex allows leading zeros** — `lib.rs:217` — "09" is accepted as major 9. Fails at lookup but parsing silently accepts it.
- [ ] **M4. Missing enumeration integration tests** — No happy-path test for the full DSL like `"3x10,23..26,5x28,34,dev"`.
- [ ] **M5. `unwrap_or(0)` silently swallows parse errors** — `lib.rs:223,251` — Minor digit parse failures default to `0` instead of reporting an error.

## LOW

- [ ] **L1. `// @formatter:off/on` comments** — `lib.rs:26,60` — IntelliJ directives, no effect with `rustfmt`.
- [ ] **L2. `DEVELOPMENT_TAG` check in range** — `lib.rs:186` — Checks `parts[1] == DEVELOPMENT_TAG` ("development") instead of `DEVELOPMENT_VERSION` ("dev"). Verify intent.
