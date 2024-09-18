</div>

<div align="center">

<br>

# 🗓️ gcal_rs: Google Calendar API 🗓️

> A blazingly fast, hand written Google calendar API in Rust.

<a href="https://docs.rs/gcal_rs/latest/gcal_rs/"> ![Docs](https://img.shields.io/docsrs/gcal_rs?color=37d4a7&logo=rust&style=for-the-badge)</a>
<a href="https://crates.io/crates/gcal_rs"> ![Crate](https://img.shields.io/crates/v/gcal_rs?color=ff4971&style=for-the-badge)</a>
<a href="/LICENSE"> ![License](https://img.shields.io/badge/license-GPL%20v3-blueviolet?style=for-the-badge)</a>
<a href="#development"> ![Status](https://img.shields.io/badge/status-WIP-informational?style=for-the-badge&color=ff69b4) </a>

[Usage](#usage)
•
[Notes](#notes)
•
[Examples](#examples)
•
[Development](#development)
<br>
[Docs](https://docs.rs/shadocal/latest/shadocal/)

</div>

## Usage

# gcal: Another Google Calendar API library for rust-lang

I wrote this by hand because I found other clients hard to use for my use-cases. This provides an API layer into the Google Calendar API that is very minimal but also mostly complete. Types are fully represented.

## Example

```rust
use gcal::*;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let access_key = std::env::args().nth(1).expect("Provide an access key");
    let now = chrono::Local::now();
    let client = Client::new(access_key);
    let client = EventClient::new(client);
    let list = client.list(now - chrono::Duration::days(1), now).await?;

    for event in &list {
        eprintln!("{} {}", event.id, event.summary);
    }
}
```

## Status

This library is being maintained by hand and is not generated from any API source e.g. OpenAPI, because I can't seem to find an example of Google providing that directly. As a result, calls may be incorrect in spots, especially where they are supplied for completeness and not used in [saturn](https://github.com/erikh/saturn) which is what this library was built to power.

If documentation is sparse, I am sorry, if you need explanations please feel free to put in a ticket.

I am happy to maintain the work within reason, but the goal is mostly to prop up saturn, and any major overhauls that would alter that charter would likely be rejected.

## Author

Shadorain <shadorain7517@gmail.com>

### Original Authors

Erik Hollensbe <erik+github@hollensbe.org> : [gcal](https://github.com/erikh/gcal)
