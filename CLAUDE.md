# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

`binman` is a Rust project (edition 2024) currently in early development. The name and purpose have not yet been defined beyond the initial `cargo new` scaffold.

## Git

Always include a `Co-Authored-By` trailer in every commit message reflecting the model that made the change, e.g.:

```
Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
```

## Commands

```bash
cargo build          # compile
cargo run            # build and run
cargo test           # run all tests
cargo test <name>    # run a single test by name (substring match)
cargo clippy         # lint
cargo fmt            # format
```
