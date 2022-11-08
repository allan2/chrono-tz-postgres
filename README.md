# chrono-tz-postgres

[![crates.io](https://img.shields.io/crates/v/chrono-tz-postgres.svg)](https://crates.io/crates/chrono-tz-postgres)
[![Released API docs](https://docs.rs/chrono-tz-postgres/badge.svg)](https://docs.rs/chrono-tz-postgres)
[![msrv 1.60](https://img.shields.io/badge/msrv-1.60-dea584.svg)](https://github.com/rust-lang/rust/releases/tag/1.60.0)

A timezone type that can be converted to and from a custom Postgres type.
This allows you to use typed timezones in PostgreSQL.

The custom Postgres enum `tz` is equivalent to the Rust enum `chrono_tz::Tz`. It can be found [here](tz.sql).

## chrono-tz-postgres in action

```rust
use chrono_tz::Tz;
use chrono_tz_postgres::TzPg;
use postgres::{Client, NoTls};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We can convert Tz into and from TzPg
    let tz = Tz::Arctic__Longyearbyen;
    let tzpg: TzPg = tz.into();
    let tz: Tz = tzpg.into();

    // Tz can be serialized, unlike TzPg
    serde_json::to_string(&tz)?;

    // but TzPg can interface with Postgres
    let mut client = Client::connect("host=localhost user=postgres", NoTls)?;
    let row = client.query_one("SELECT timezone FROM foo LIMIT 1", &[])?;
    let tz: TzPg = row.get(0);

    // Convert back to Tz to do more
    let tz: Tz = tz.into();
    println!("{tz}");

    Ok(())
}
```
