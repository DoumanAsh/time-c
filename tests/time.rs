use time_c::sys::{get_time, utc_time};
use time_c::format::Rfc3339;

#[test]
fn should_get_current_time() {
    let now = get_time().expect("get time");
    assert_ne!(now, 0);
    let tm = utc_time(&now).expect("parse time");
    println!("tm={}", Rfc3339(&tm));
    assert_ne!(tm.tm_year, 0);
}
