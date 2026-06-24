# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

`binman` is a Rust CLI tool that notifies the user on bin collection days, telling them which bins to put out. It is designed to run as a daily cron job. See `SPEC.md` for the full specification.

## Development approach

This project uses spec-driven development. `SPEC.md` is the source of truth. Before implementing anything:
1. Confirm the relevant spec section is complete and agreed
2. Translate the spec into Rust type/trait skeletons (`todo!()` bodies) that compile
3. Write tests against the skeletons
4. Implement to make tests pass
5. If reality diverges from the spec, update the spec deliberately — never as a side-effect

When anything is ambiguous — requirements, behaviour, design — ask the user for clarification before proceeding. Err strongly on the side of asking.

Future nice-to-haves that are explicitly out of scope belong in the `Future Considerations` section of `SPEC.md`, not here.

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
