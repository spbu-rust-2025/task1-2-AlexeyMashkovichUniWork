use std::io;

fn main() -> io::Result<()> {
	let mut buffer : String;
	let stdin = io::stdin();
	loop {
		buffer = "".to_string();
		stdin.read_line(&mut buffer)?;
		let parsed_buffer = buffer.trim();
		println!("{parsed_buffer}");
		if parsed_buffer == "-1" {
			break;
		}
	}
	Ok(())
}
