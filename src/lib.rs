//!Wrapper for time functions of C standard library
//!
//!This provides minimal and uniform API to use in Rust without need for cumbersome crates like time
#![no_std]
#![warn(missing_docs)]
#![allow(clippy::style)]

pub mod format;
pub mod sys;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
///Normalized [tm](sys/struct.tm.html)
pub struct Time {
    ///Seconds after the minute. Range 0-60
    pub sec: u8,
    ///Minutes after the hour. Range 0-59
    pub min: u8,
    ///Hours since midnight. Range 0-23
    pub hour: u8,
    ///Day of the month. Range 1-31
    pub month_day: u8,
    ///Months since January. Range 1-12
    pub month: u8,
    ///Year
    pub year: u16,
    ///days since Sunday. Range 0-6
    pub week_day: u8,
    ///Days since January 1. Range 0-365
    pub day: u16,
    ///Indicates Daylight Saving Time presence.
    pub is_dst: bool,
}

impl Time {
    #[inline(always)]
    ///Attempts to parse provided unix time and returns new instance of self
    pub fn parse_unix(secs: &sys::time_t) -> Option<Self> {
        sys::parse_unix(secs).map(|time| time.normalize())
    }

    #[inline(always)]
    ///Gets current UTC time
    pub fn now_utc() -> Option<Self> {
        sys::get_time().and_then(|time| Self::parse_unix(&time))
    }

    #[inline(always)]
    ///Gets RFC-3339 formatter for this instance.
    pub fn rfc3339(&self) -> format::Rfc3339<'_, Self> {
        format::Rfc3339(self)
    }
}
