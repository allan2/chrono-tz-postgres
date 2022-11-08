use chrono_tz::Tz;
use chrono_tz_postgres::TzPg;

fn main() {
    // We can convert between Tz and TzPg
    let tz = Tz::Arctic__Longyearbyen;
    let tzpg = TzPg::from(tz);
    let tz: Tz = tzpg.into();
    println!("{}", tz);
}
