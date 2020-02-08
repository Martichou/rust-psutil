//! Utility methods, mostly for dealing with IO.

use std::io;
use std::time::Duration;

use crate::Percent;

#[cfg(target_os = "linux")]
macro_rules! try_parse {
	($field:expr) => {
		try_parse!($field, std::str::FromStr::from_str)
	};
	($field:expr, $from_str:path) => {
		$from_str($field).map_err(|_| {
			std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				format!("Could not parse {:?}", $field),
				)
			})?
	};
}

#[cfg(target_os = "linux")]
pub(crate) fn not_found(key: &str) -> io::Error {
	io::Error::new(io::ErrorKind::NotFound, format!("{} not found", key))
}

pub(crate) fn invalid_data(message: &str) -> io::Error {
	io::Error::new(io::ErrorKind::InvalidData, message)
}

// TODO: fix casting
// TODO: use nightly div_duration_f32
#[allow(clippy::unnecessary_cast)]
pub(crate) fn div_duration_f32(lhs: Duration, rhs: Duration) -> Percent {
	((lhs.as_nanos() as f64 / rhs.as_nanos() as f64) * 100.0) as f32
}
