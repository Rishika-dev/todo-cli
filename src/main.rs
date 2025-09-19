mod cli;

use cli::command::Command;
use cli::file_ops::*;
use cli::utils::*;
use colored::*;
use std::io;

fn main() -> io::Result<()> {
    loop {
        println!("{}", "***** Welcome to TODO CLI *****".bright_blue().bold());
        println!("{}", "1. Create File".yellow());
        println!("{}", "2. Write on a file".yellow());
        println!("{}", "3. Append on a file".yellow());
        println!("{}", "4. Exit".red());

        let choice = read_line("Enter your choice: ")?;
        let choice_num = choice.trim().parse::<u32>().unwrap_or(0);

        let command = match choice_num {
            1 => {
                let file_name = read_line("Enter file name: ")?;
                Command::CreateFile(file_name)
            }
            2 => {
                let file_name = read_line("Enter file name: ")?;
                let content = read_multiline_input("Enter content to write (finish with empty line):")?;
                Command::WriteFile(file_name, content)
            }
            3 => {
                let file_name = read_line("Enter file name: ")?;
                let content = read_multiline_input("Enter content to append (finish with empty line):")?;
                Command::AppendFile(file_name, content)
            }
            4 => break,
            _ => Command::Invalid,
        };

        match command {
            Command::CreateFile(file_name) => {
                file_create(&file_name)?;
                println!("{}", "File created successfully!".green());
            }
            Command::WriteFile(file_name, content) => {
                file_write(&file_name, &content)?;
                println!("{}", "File written successfully!".green());
            }
            Command::AppendFile(file_name, content) => {
                file_append(&file_name, &content)?;
                println!("{}", "Content appended successfully!".green());
            }
            Command::Invalid => println!("{}", "Invalid choice!".red()),
        }
    }

    Ok(())
}
