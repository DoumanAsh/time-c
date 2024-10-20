use time_c::sys::{get_time, utc_time};
use time_c::format::Rfc3339;

#[test]
fn should_get_current_time() {
    let now = get_time().expect("get time");
    assert_ne!(now, 0);
    let tm = utc_time(&now).expect("parse time");
    let tm_repr = Rfc3339(&tm).to_string();
    println!("tm_repr={tm_repr}");
    assert_ne!(tm.tm_year, 0);

    let normalized = tm.normalize();
    assert_eq!(tm.tm_sec, normalized.sec as _);
    assert_eq!(tm.tm_min, normalized.min as _);
    assert_eq!(tm.tm_hour, normalized.hour as _);
    assert_eq!(tm.tm_mday, normalized.month_day as _);
    assert_eq!(tm.tm_mon.saturating_add(1), normalized.month as _);
    assert_eq!(tm.tm_year.saturating_add(1900), normalized.year as _);
    assert_eq!(tm.tm_wday, normalized.week_day as _);
    assert_eq!(tm.tm_yday, normalized.day as _);

    let normalized_repr = normalized.rfc3339().to_string();
    println!("normalized={tm_repr}");
    assert_eq!(tm_repr, normalized_repr);
}
