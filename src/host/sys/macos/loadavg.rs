use crate::host::LoadAvg;

use crate::{Result};

extern "C" {
	fn getloadavg(loadavg: &[f64], nelem: i32) -> i32;
}

pub fn loadavg() -> Result<LoadAvg> {
	let loads: [f64; 3] = [0.0; 3];
	unsafe {
		getloadavg(&loads, 3);
	}
	Ok(LoadAvg {
		one: loads[0],
		five: loads[1],
		fifteen: loads[2],
	})
}