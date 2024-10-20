//! Formatting module

use core::fmt;

use crate::sys::tm;

#[repr(transparent)]
///RFC-3339 encoder for tm
pub struct Rfc3339<'a>(pub &'a tm);

impl fmt::Display for Rfc3339<'_> {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tm {
            tm_sec,
            tm_min,
            tm_hour,
            tm_mday,
            mut tm_mon,
            mut tm_year,
            ..
        } = self.0;
        //month starts from 0 so add 1
        tm_mon = tm_mon.wrapping_add(1);
        //tm_year is relative to 1900
        tm_year = tm_year.saturating_add(1900);
        fmt.write_fmt(format_args!("{tm_year:04}-{tm_mon:02}-{tm_mday:02}T{tm_hour:02}:{tm_min:02}:{tm_sec:02}Z"))
    }
}
