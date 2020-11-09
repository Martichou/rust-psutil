use std::io::Error;
use std::fs;

pub fn get_machine_id() -> Result<String, Error> {
	const DBUS_F: &str = "/etc/machine-id";
	const DBUS_S: &str = "/var/lib/dbus/machine-id";

	match read_uuid(DBUS_F) {
		Ok(machine_id) => Ok(machine_id),
		Err(_) => Ok(read_uuid(DBUS_S)?),
	}
}

fn read_uuid(path: &str) -> Result<String, Error> {
	let content = fs::read_to_string(path)?;
	Ok(content.trim().to_string())
}