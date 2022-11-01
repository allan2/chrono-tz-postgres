# chrono-tz-postgres

[![crates.io](https://img.shields.io/crates/v/chrono-tz-postgres.svg)](https://crates.io/crates/chrono-tz-postgres)
[![Released API docs](https://docs.rs/chrono-tz-postgres/badge.svg)](https://docs.rs/chrono-tz-postgres)
[![msrv 1.61](https://img.shields.io/badge/msrv-1.61-dea584.svg)](https://github.com/rust-lang/rust/releases/tag/1.61.0)

A timezone type that can be converted to and from a custom Postgres type.

This is the same as `chrono_tz::Tz`, but with `ToSql` and `FromSql` implemented.
The custom Postgres type `tz` can be found [here](postgres_enum.sql).

## chrono-tz-postgres in action

```rust
pub use chrono_tz::Tz;
use chrono_tz_postgres::TzPg;
use postgres::{Client, NoTls};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We can convert Tz into and from TzPg
    let tz = Tz::Arctic__Longyearbyen;
    let tzpg: TzPg = tz.into();
    let tz: Tz = tzpg.into();

    // Tz can be serialized, unlike TzPg
    serde_json::to_string(&tz).unwrap();

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
