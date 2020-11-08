use std::time::Duration;
use libc::{c_int, c_void};

use crate::{Error, Result};

#[repr(C)]
#[derive(Debug)]
pub struct ut_tv {
    pub tv_sec: i32,
    pub tv_usec: i32,
}

impl Default for ut_tv {
    fn default() -> ut_tv {
        ut_tv {
            tv_sec: 0,
            tv_usec: 0,
        }
    }
}

extern "C" {
	fn clock_gettime(clk_id: c_int, tp: *mut c_void) -> c_int;
}

/// New function, not in Python psutil.
pub fn uptime() -> Result<Duration> {
	let mut uptime: ut_tv = Default::default();
	let xuptime: *mut c_void = &mut uptime as *mut _ as *mut c_void;
	unsafe {
		if 0 != clock_gettime(4, xuptime) {
			return Err(Error::UnsafeError);
		}
	}
	Ok(Duration::from_secs(uptime.tv_sec as u64))
}
