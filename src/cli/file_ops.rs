use std::fs::{File, OpenOptions};
use std::io::{self, Write};

pub fn file_create(file_name: &str) -> io::Result<()> {
    File::create(file_name)?;
    Ok(())
}

pub fn file_write(file_name: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn file_append(file_name: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
