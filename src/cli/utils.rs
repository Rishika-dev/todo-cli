use std::io::{self, Write};

pub fn read_line(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?; // ensure prompt appears before input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn read_multiline_input(prompt: &str) -> io::Result<String> {
    println!("{}", prompt);
    let mut content = String::new();
    loop {
        let line = read_line("")?;
        if line.is_empty() {
            break;
        }
        content.push_str(&line);
        content.push('\n');
    }
    Ok(content)
}
