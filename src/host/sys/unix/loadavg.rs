use crate::host::LoadAvg;

use crate::Result;

use nix::libc;

pub fn loadavg() -> Result<LoadAvg> {
	let mut data: [libc::c_double; 3] = [0.0, 0.0, 0.0];
	unsafe {
		libc::getloadavg(data.as_mut_ptr(), 3);
	}
	Ok(LoadAvg {
		one: data[0],
		five: data[1],
		fifteen: data[2],
	})
}
