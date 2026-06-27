# binman

A tiny command-line tool that reminds you to put the bins out.

`binman` checks the [Greater Cambridge Shared Waste](https://www.scambs.gov.uk/bins/) collection
schedule for your address and, when there's a collection tomorrow, sends you a push notification
telling you exactly which bins to put out. It's built to run as a daily cron job and to be silent
on every day that isn't a bin day.

## Why this exists

Bin collections happen in the morning, so the bins have to go out **the night before**. It's easy
to forget which week is recycling, which is garden waste, and whether this is the week you got the
food caddy out too. `binman` takes the council's own schedule, works out whether tomorrow is a
collection day, and nudges you on your phone — twice:

- a **morning** heads-up so you know it's coming, and
- an **evening** reminder so you actually walk the bins down the drive.

On any other day, it does nothing at all.

## How it works

1. It looks up your property's collection schedule from the Greater Cambridge Shared Waste API
   (the same data behind the council's "check your bin day" page), using your property's UPRN.
2. It checks whether there is a collection scheduled for **tomorrow**.
3. If there is, it sends a notification — via [ntfy](https://ntfy.sh/) — listing the bins for that
   collection.
4. If there isn't, it exits quietly with no notification.

> **Important:** Both the `morning` and `evening` commands check *tomorrow's* schedule, because
> "bin day" means "there's a collection tomorrow, so put the bins out tonight". The two commands
> differ only in the wording of the notification — neither one looks at today's collection.

### The bins it knows about

| Council round | What `binman` calls it |
| --- | --- |
| `DOMESTIC` | general waste |
| `RECYCLE` | recycling |
| `ORGANIC` | garden waste |
| `FOOD` | food waste |

Any round type the council adds that `binman` doesn't recognise is passed through to the
notification using the council's own label, so you'll still be told about it.

## Requirements

- A property within the **Greater Cambridge Shared Waste** collection area (South Cambridgeshire /
  Cambridge City). Other councils are not supported.
- A device that can run a daily cron job (a Raspberry Pi, a home server, a VPS — anything always-on).
- An [ntfy](https://ntfy.sh/) topic to receive notifications. You can use the free public
  `ntfy.sh` server or self-host. Install the ntfy app on your phone and subscribe to your topic.
- [Rust](https://www.rust-lang.org/tools/install) (stable, 2024 edition) to build from source.

## Installation

Clone the repository and build a release binary:

```bash
git clone https://github.com/timleach635/binman.git
cd binman
cargo build --release
```

The compiled binary will be at `target/release/binman`. Copy it somewhere on your `PATH`
(for example `~/.local/bin/` or `/usr/local/bin/`).

## Configuration

`binman` is configured entirely through environment variables:

| Variable | Required | Description |
| --- | --- | --- |
| `BINMAN_UPRN` | **Yes** | The UPRN (Unique Property Reference Number) of your property. Look yours up at [findmyaddress.co.uk](https://www.findmyaddress.co.uk/). |
| `BINMAN_NTFY_TOPIC` | **Yes** | The ntfy topic to publish notifications to, e.g. `my-bin-reminders`. Pick something hard to guess if you're using the public `ntfy.sh` server. |
| `BINMAN_NTFY_URL` | No | Base URL of your ntfy server. Defaults to `https://ntfy.sh`. Set this if you self-host ntfy. |

If a required variable is missing, `binman` exits with a non-zero status and a clear error message
(and, where possible, sends an error notification — see [Error handling](#error-handling)).

### Finding your UPRN

Go to [findmyaddress.co.uk](https://www.findmyaddress.co.uk/), search for your address, and copy
the UPRN it shows. It's a long number, e.g. `100090161086`.

## Usage

`binman` takes a single argument — the notification to send:

```bash
binman morning   # "Bin day today" — a heads-up that bins go out tonight
binman evening   # "Put your bins out!" — the evening prompt
```

Both commands check whether there's a collection **tomorrow**. If there is, you get a notification
like:

> **Bin day today**
> Putting out: recycling, garden waste

or, in the evening:

> **Put your bins out!**
> Tonight: recycling, garden waste

If there's no collection tomorrow, the command prints nothing and sends nothing.

A quick manual test (substitute your own values):

```bash
BINMAN_UPRN=100090161086 \
BINMAN_NTFY_TOPIC=my-bin-reminders \
  binman morning
```

## Running on a schedule

`binman` is meant to be driven by cron. Add two entries — a morning heads-up at 7am and an evening
reminder at 7pm:

```cron
# m h  dom mon dow  command
0  7   *   *   *    binman morning
0  19  *   *   *    binman evening
```

Cron runs with a minimal environment, so set the configuration variables where the job can see
them. For example, at the top of your crontab:

```cron
BINMAN_UPRN=100090161086
BINMAN_NTFY_TOPIC=my-bin-reminders

0  7   * * *  /usr/local/bin/binman morning
0  19  * * *  /usr/local/bin/binman evening
```

(If you self-host ntfy, also add `BINMAN_NTFY_URL=https://ntfy.example.com`.)

## Error handling

`binman` is designed to fail loudly, because a reminder you never get is worse than no reminder at
all. On any failure it will:

1. Send an ntfy notification describing the problem (so you find out even though it's running
   unattended in cron),
2. Write the error to standard error, and
3. Exit with a non-zero status code.

The cases it handles this way include a missing required environment variable, a network failure
reaching the collection API, an unexpected or malformed response from the API, and a failure
reaching ntfy. If ntfy itself can't be reached — or if `BINMAN_NTFY_TOPIC` isn't set — then standard
error is the only place the failure can be reported.

## Development

This project follows spec-driven development. [`SPEC.md`](./SPEC.md) is the source of truth for
behaviour; if you're changing what `binman` does, start there.

Common commands:

```bash
cargo build          # compile
cargo run -- morning # build and run (note the -- before the argument)
cargo test           # run all tests
cargo test <name>    # run a single test by name (substring match)
cargo clippy         # lint
cargo fmt            # format
```

Some tests are integration tests that hit the network or send a real notification; these are marked
`#[ignore]` and are skipped by default. Run them explicitly with `cargo test -- --ignored`.

### Project layout

| File | Responsibility |
| --- | --- |
| `src/main.rs` | CLI entry point, argument parsing, and the core "is tomorrow a bin day?" logic. |
| `src/config.rs` | Reads and validates configuration from environment variables. |
| `src/collection.rs` | The `Collection` / `BinType` types and the `CollectionSource` trait. |
| `src/bartec.rs` | The `CollectionSource` implementation that talks to the council's API. |
| `src/notify.rs` | The `Notifier` trait and its ntfy implementation. |
| `src/error.rs` | The shared error type. |

The data source sits behind the `CollectionSource` trait so it can be swapped out — for example to
add local caching of the schedule — without touching the rest of the program.

## Data source

The schedule comes from an undocumented but stable public API operated by Greater Cambridge Shared
Waste (powered by Bartec):

```
GET https://servicelayer3c.azure-api.net/wastecalendar/collection/search/{UPRN}/?authority=CCC&numberOfCollections=255
```

Dates from the API are returned in UTC and converted to your local date, so collections aren't
accidentally shifted by a day during British Summer Time.

## Licence

See the repository for licensing information.
