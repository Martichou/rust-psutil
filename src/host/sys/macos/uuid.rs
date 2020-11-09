use std::io::{Error, ErrorKind};
use std::process::Command;

/// Return machine id
pub fn get_machine_id() -> Result<String, Error> {
	let output = Command::new("ioreg")
		.args(&["-rd1", "-c", "IOPlatformExpertDevice"])
		.output()?;
	let content = String::from_utf8_lossy(&output.stdout);
	extract_id(&content)
}

fn extract_id(content: &str) -> Result<String, Error> {
	let lines = content.split('\n');
	for line in lines {
		if line.contains("IOPlatformUUID") {
			let k: Vec<&str> = line.rsplitn(2, '=').collect();
			let id = k[0].trim_matches(|c: char| c == '"' || c.is_whitespace());
			return Ok(id.to_string());
		}
	}
	Err(Error::new(
		ErrorKind::Other,
		"No matching IOPlatformUUID in `ioreg -rd1 -c IOPlatformExpertDevice` command.",
	))
}
