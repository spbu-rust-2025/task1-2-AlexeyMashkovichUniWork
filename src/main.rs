use std::io;

fn main() -> io::Result<()> {
    let mut buffer: String;
    let stdin = io::stdin();
    let mut sum: u128 = 0;
    loop {
        buffer = "".to_string();
        stdin.read_line(&mut buffer)?;
        let parsed_buffer = buffer.trim();
        if parsed_buffer == "-1" {
            println!("{sum}");
            break;
        }
        match parsed_buffer.parse::<u128>() {
            Ok(number) => sum += number,
            Err(_) => {
                println!("NaN");
                break;
            }
        }
    }
    Ok(())
}
