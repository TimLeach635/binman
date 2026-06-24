# binman — Specification

## Overview

`binman` is a CLI tool that sends push notifications on bin collection days, telling the user which bins to put out. It is designed to run as a cron job and queries the Greater Cambridge Waste website for the collection schedule.

## User-facing behaviour

- On a collection day, `binman` sends **two** notifications:
  - A **morning** notification: a heads-up that today is a bin day and which bins are collected
  - An **evening** notification (target: 7pm): a prompt to actually take the bins out
- On non-collection days, `binman` produces **no output and sends no notification**
- The collection schedule (which bins, which dates) is always retrieved from the data source — it is never inferred or hardcoded

## Invocation

`binman` takes a single required argument indicating which notification to send:

```
binman morning
binman evening
```

The cron schedule that achieves the target behaviour:

```cron
0  7  * * *  binman morning
0 19  * * *  binman evening
```

## Configuration

All configuration is via environment variables:

| Variable | Description |
|---|---|
| `BINMAN_ADDRESS` | The address or postcode to look up on the council website. Exact format to be confirmed when the data source is investigated. |
| `BINMAN_NTFY_TOPIC` | The ntfy topic name to publish notifications to (e.g. `my-bin-reminders`) |
| `BINMAN_NTFY_URL` | Base URL of the ntfy server. Defaults to `https://ntfy.sh` if unset. |

`binman` must exit with a non-zero status code and a clear error message if any required variable is missing.

## Data source

The collection schedule is retrieved from the **Greater Cambridge Waste** service:

> https://www.greatercambridgewaste.org/find-your-household-bin-collection-day

Before implementing, investigate whether this service exposes a public API. If it does, prefer the API. If not, scrape the HTML response. The implementation must be isolated behind a trait so the data source can be swapped without changing the rest of the program.

The three bin types in use are: **general waste**, **recycling**, and **garden waste**. Collections alternate weekly: general waste one week, recycling and garden waste the next.

## Notifications

Notifications are sent via **[ntfy](https://ntfy.sh/)** using its HTTP API (a single POST request). 

Message content (exact wording TBD, but intent):

- **Morning**: "Bin day today — putting out: [bin list]"
- **Evening**: "Don't forget to put your bins out — tonight: [bin list]"

## Error handling

On any failure, `binman` must:
1. Send an ntfy notification to `BINMAN_NTFY_TOPIC` describing the problem
2. Write the error to stderr
3. Exit with a non-zero status code

Failure cases include:
- Missing required env vars
- Network failure reaching the data source
- Unexpected response from the data source (e.g. page format has changed, scraper needs updating)
- Network failure reaching ntfy (in this case, only stderr output is possible)

If `BINMAN_NTFY_TOPIC` is not set, the ntfy notification cannot be sent; stderr is the only output in that case.

## Non-goals (v1)

- Caching of the collection schedule (see Future Considerations)
- Supporting councils other than Greater Cambridge Waste
- Multiple addresses
- Configurable notification wording

## Future considerations

- **Local schedule caching**: cache the retrieved schedule (e.g. for 30 days) to a local file to reduce network usage on constrained devices such as a Raspberry Pi. The data source trait boundary is designed to make this straightforward to add.
