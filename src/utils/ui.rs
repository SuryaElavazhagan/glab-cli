use std::io;

/// Prompts for input and returns it.
pub fn prompt(message: &str) -> io::Result<String> {
  loop {
    print!("{}: ", message);
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let input = buf.trim();
    if !input.is_empty() {
      return Ok(input.to_owned());
    }
  }
}
