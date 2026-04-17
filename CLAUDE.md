# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust library crate (`wildfly_container_versions`) for managing WildFly container versions. It maps short version identifiers (e.g., "35") to full container image references on Docker Hub (`docker.io/jboss/wildfly`) and Quay (`quay.io/wildfly/wildfly`). Consumed by CLI tools like [wado](https://github.com/hpehl/wado).

## Build & Test Commands

```bash
cargo build          # build the library
cargo test           # run all tests
cargo test <name>    # run a single test by name (e.g., cargo test single_version_ok)
cargo clippy         # lint
cargo fmt --check    # check formatting
```

## Architecture

Single-file library (`src/lib.rs`) with no binary targets:

- **`VERSIONS`** — a `lazy_static` `BTreeMap<u16, WildFlyContainer>` mapping identifier keys to container metadata. The identifier is `major * 10 + minor`.
- **`WildFlyContainer`** — the core struct holding version, core_version, suffix, repository, and platforms. Provides parsing methods for version expressions.
- **Version expression parser** — `enumeration()` → `range()` / `versions()` → `version()` / `multiplier()`. Parses a mini-DSL: `"3x10,23..26,5x28,34,dev"`.
- **`WILDFLY_DEV`** — special dev container (identifier 0) representing a build-from-source variant.

## Adding a New WildFly Version

Add a new `m.insert(identifier(...), WildFlyContainer::new(...))` entry to the `VERSIONS` lazy_static block in `src/lib.rs`, then update test assertions that depend on version counts or the last version (e.g., `range_from`, `range_all`).

## Release Process

Run `./release.sh <semver>` which bumps the version via `cargo bump`, commits, tags (`v<semver>`), and pushes to trigger the GitHub release workflow.
