use chrono_tz::Tz;
use chrono_tz_postgres::TzPg;
use postgres::{Client, NoTls};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We can convert between Tz and TzPg
    let tz = Tz::Arctic__Longyearbyen;
    let tzpg = TzPg::from(tz);
    let tz: Tz = tzpg.into();
    println!("{}", tz);

    // Tz can be serialized, unlike TzPg
    serde_json::to_string(&tz)?;

    // but TzPg can interface with Postgres
    let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

    // timezone the column name, whose type is `tz`
    // `tz` is a custom Postgres type
    let row = client.query_one("SELECT timezone FROM foo LIMIT 1", &[])?;
    let tz: TzPg = row.get(0);

    // Convert back to Tz to do more
    let tz: Tz = tz.into();
    println!("{}", tz);

    Ok(())
}
