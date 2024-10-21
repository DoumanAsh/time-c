//!Raw system types and functions
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::Time;

use core::{mem, ptr, time};
use core::ffi::{c_void, c_int, c_long};

///Alias to time_t.
///
///This crate supports 64bit time only
///
///## Note
///
///Your system uses 32bit for time()? Don't use this crate then
pub type time_t = i64;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
///C definition of decoded time.
pub struct tm {
    ///Seconds after the minute. Range 0-60
    pub tm_sec: c_int,
    ///Minutes after the hour. Range 0-59
    pub tm_min: c_int,
    ///Hours since midnight. Range 0-23
    pub tm_hour: c_int,
    ///Day of the month. Range 1-31
    pub tm_mday: c_int,
    ///Months since January. Range 0-11
    pub tm_mon: c_int,
    ///Years since 1900
    pub tm_year: c_int,
    ///days since Sunday. Range 0-6
    pub tm_wday: c_int,
    ///days since January 1. Range 0-365
    pub tm_yday: c_int,
    /// Daylight Saving Time flag. Non-zero value indicates DST is present.
    pub tm_isdst: c_int,
    //Other fields are non-standard and depend on platform
    //So don't care about these fields
    _reserved: mem::MaybeUninit<[u8; mem::size_of::<c_long>() + mem::size_of::<*const c_void>() + mem::size_of::<c_int>()]>,
}

impl tm {
    ///Normalizes time to a more convenient struct for interpreting time components.
    pub const fn normalize(&self) -> Time {
        let tm {
            tm_sec,
            tm_min,
            tm_hour,
            tm_mday,
            tm_mon,
            tm_year,
            tm_wday,
            tm_yday,
            tm_isdst,
            ..
        } = self;

        Time {
            sec: *tm_sec as _,
            min: *tm_min as _,
            hour: *tm_hour as _,
            month_day: *tm_mday as _,
            month: (*tm_mon as u8).saturating_add(1),
            year: (*tm_year as u16).saturating_add(1900),
            week_day: *tm_wday as _,
            day: *tm_yday as _,
            is_dst: *tm_isdst > 0,
        }
    }
}

extern "C" {
    #[cfg_attr(windows, link_name = "_time64")]
    ///Raw C API function to access time
    pub fn time(time: *mut time_t) -> time_t;
}

///Gets current UTC time in seconds, if available.
pub fn get_time() -> Option<time_t> {
    let result = unsafe {
        time(ptr::null_mut())
    };

    match result {
        -1 => None,
        time => Some(time)
    }
}


#[cfg(windows)]
///Parses UTC time into tm struct, if possible
pub fn parse_unix(timer: &time_t) -> Option<tm> {
    extern "C" {
        #[link_name = "_gmtime64_s"]
        pub fn gmtime_s(buf: *mut tm, timer: *const time_t) -> c_int;
    }

    let mut tm = mem::MaybeUninit::uninit();
    let res = unsafe {
        gmtime_s(tm.as_mut_ptr(), timer)
    };
    match res {
        0 => unsafe {
            Some(tm.assume_init())
        },
        _ => None,
    }
}

#[cfg(not(windows))]
///Parses UTC time into tm struct, if possible
pub fn parse_unix(timer: &time_t) -> Option<tm> {
    extern "C" {
        pub fn gmtime_r(timer: *const time_t, buf: *mut tm) -> *mut tm;
    }

    let mut tm = mem::MaybeUninit::uninit();
    let res = unsafe {
        gmtime_r(timer, tm.as_mut_ptr())
    };
    if res.is_null() {
        None
    } else {
        unsafe {
            Some(tm.assume_init())
        }
    }
}

#[cfg(windows)]
///Gets UTC time, if available
pub fn utc_now() -> Option<time::Duration> {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct FILETIME {
        pub dwLowDateTime: u32,
        pub dwHighDateTime: u32,
    }

    #[repr(C)]
    pub union ULARGE_INTEGER {
        file_time: FILETIME,
        integer: u64,
    }

    extern "system" {
        fn GetSystemTimeAsFileTime(time: *mut FILETIME);
    }

    let time = unsafe {
        let mut time = mem::MaybeUninit::<ULARGE_INTEGER>::uninit();
        GetSystemTimeAsFileTime(ptr::addr_of_mut!((*time.as_mut_ptr()).file_time));
        time.assume_init().integer.saturating_sub(116_444_736_000_000_000)
    };

    Some(time::Duration::new(
        time / 10_000_000,
        ((time % 10_000_000) * 100) as _
    ))
}

#[cfg(unix)]
///Gets UTC time, if available
pub fn utc_now() -> Option<time::Duration> {
    #[repr(C)]
    struct timespec {
        tv_sec: time_t,
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        tv_nanos: i64,
        #[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))]
        tv_nanos: c_long,
    }

    extern "C" {
        fn timespec_get(time: *mut timespec, base: c_int) -> c_int;
    }

    let time = unsafe {
        let mut time = mem::MaybeUninit::uninit();
        if timespec_get(time.as_mut_ptr(), 1) == 0 {
            return None;
        }
        time.assume_init()
    };

    Some(time::Duration::new(time.tv_sec as _, time.tv_nanos as _))
}

