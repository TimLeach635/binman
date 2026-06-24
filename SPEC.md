# binman — Specification

## Overview

`binman` is a CLI tool that sends push notifications on bin collection days, telling the user which bins to put out. It is designed to run as a cron job and queries the Greater Cambridge Waste website for the collection schedule.

## User-facing behaviour

- Collections happen in the morning. Bins must be put out the **night before**.
- A day is **"bin day"** if there is a collection scheduled for the **following calendar day**.
- On bin day, `binman` sends **two** notifications:
  - A **morning** notification (7am): a heads-up that today is bin day and which bins go out tonight
  - An **evening** notification (7pm): a prompt to actually put the bins out
- **Both** morning and evening modes check whether tomorrow has a collection. The only difference between them is the notification wording.
- If tomorrow has no collection, `binman` produces **no output and sends no notification**, regardless of mode.
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

| Variable | Required | Description |
|---|---|---|
| `BINMAN_UPRN` | Yes | The UPRN (Unique Property Reference Number) of the property. Look yours up at findmyaddress.co.uk. |
| `BINMAN_NTFY_TOPIC` | Yes | The ntfy topic name to publish notifications to (e.g. `my-bin-reminders`) |
| `BINMAN_NTFY_URL` | No | Base URL of the ntfy server. Defaults to `https://ntfy.sh` if unset. |

`binman` must exit with a non-zero status code and a clear error message if any required variable is missing.

## Data source

The collection schedule is retrieved from an undocumented but stable public API operated by Greater Cambridge Shared Waste (powered by Bartec) and hosted on Azure API Management.

**Collection schedule endpoint:**
```
GET https://servicelayer3c.azure-api.net/wastecalendar/collection/search/{UPRN}/?authority=CCC&numberOfCollections=255
```

Response: a JSON object containing a `collections` array. Each element has:
- `date` — ISO 8601 datetime (e.g. `2024-03-18T00:00:00Z`)
- `roundTypes` — array of bin type strings

**Bin type mapping:**

| API value | Bin |
|---|---|
| `DOMESTIC` | General waste (black bin) |
| `RECYCLE` | Recycling (blue bin) |
| `ORGANIC` | Garden waste (green bin) |

The authority code `CCC` is hardcoded (Cambridge City Council). Supporting other authority codes is a non-goal for v1.

The data source implementation must be isolated behind a trait so it can be swapped without changing the rest of the program (e.g. to add caching in future).

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
- Network failure reaching the data source API
- Unexpected or malformed response from the data source API
- Network failure reaching ntfy (in this case, only stderr output is possible)

If `BINMAN_NTFY_TOPIC` is not set, the ntfy notification cannot be sent; stderr is the only output in that case.

## Non-goals (v1)

- Caching of the collection schedule (see Future Considerations)
- Supporting councils other than Greater Cambridge Waste
- Multiple addresses
- Configurable notification wording

## Future considerations

- **Local schedule caching**: cache the retrieved schedule (e.g. for 30 days) to a local file to reduce network usage on constrained devices such as a Raspberry Pi. The data source trait boundary is designed to make this straightforward to add.
