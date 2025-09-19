use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Write};
use colored::*;
fn main() -> std::io::Result<()> {
    let mut user_input = String::new();

    println!("{}", "***** Welcome to TODO CLI *****".bright_blue().bold());
    println!("{}", "1. Create File".yellow());
    println!("{}", "2. Write on a file".yellow());
    println!("{}", "3. Append on a file".yellow());

    io::stdin().read_line(&mut user_input).expect("Failed to process!");

    match user_input.trim().parse().expect("Invalid integer") {
        1 =>{
        println!("You have selected file creation");
        let mut file_name = String::new();
        println!("Enter your file name:");
        io::stdin().read_line(&mut file_name).expect("Invalid");
        file_create(&file_name.trim())?;
        },    
        2 => {
        println!("You have selected write on a file");

        let mut file_name = String::new();
        println!("{}",file_name);
        println!("Enter your file name:");
        io::stdin().read_line(&mut file_name).expect("Invalid");
        
        let mut content = String::new();
        println!("Enter your file content to write:");
        io::stdin().read_line(&mut content).expect("Invalid");

        let mut file = File::create(file_name.trim())?;
        file_write(&(content + "\n"), &mut file)?;

        },
        3 => {
        println!("You have selected append on a file");
        let mut file_name = String::new();
        println!("Enter your file name:");
        io::stdin().read_line(&mut file_name).expect("Invalid");
        
        let mut content = String::new();
        println!("Enter your file content to append:");
        io::stdin().read_line(&mut content).expect("Invalid");
        file_append(&(content + "\n"), &file_name.trim())?;

        }
        _=> println!("Invalid response"),
    }
    
    Ok(())
}

fn file_create(file_name: &str) -> std::io::Result<File> {
    let file = File::create(file_name)?;
    Ok(file)
}

fn file_write(content: &str, file: &mut File) -> std::io::Result<()> {
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn file_append(content: &str,file: &str) -> std::io::Result<()>
{
    let mut file_to_append = OpenOptions::new()
    .create(false)
    .append(true)
    .open(file)?;

    file_write(content, &mut file_to_append)
}
