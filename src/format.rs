//! Formatting module

use core::fmt;

use crate::Time;
use crate::sys::tm;

#[repr(transparent)]
///RFC-3339 encoder for tm
pub struct Rfc3339<'a, T>(pub &'a T);

impl fmt::Display for Rfc3339<'_, tm> {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let normalized = self.0.normalize();
        fmt::Display::fmt(&Rfc3339(&normalized), fmt)
    }
}

impl fmt::Display for Rfc3339<'_, Time> {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Time {
            sec,
            min,
            hour,
            month_day,
            month,
            year,
            ..
        } = self.0;

        fmt.write_fmt(format_args!("{year:04}-{month:02}-{month_day:02}T{hour:02}:{min:02}:{sec:02}Z"))
    }
}
