use crate::{input, utility};
use std::io;

pub enum MetaCommandResult {
    MetaCommandExit,
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

pub fn do_meta_command(input_buffer_command: &String) -> MetaCommandResult {
    return if input_buffer_command == ".exit" {
        MetaCommandResult::MetaCommandExit
    } else if input_buffer_command == ".help" {
        MetaCommandResult::MetaCommandSuccess
    } else {
        MetaCommandResult::MetaCommandUnrecognizedCommand
    };
}

// Print prompt message
pub fn print_prompt() -> () {
    println!("db > ")
}

// Reads an input line from the cli
pub fn read_input(input_buffer: &mut input::InputBuffer) -> () {
    let mut line: String = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(n) => {
            input_buffer.buffer = utility::string_util::remove_newline(line);
            input_buffer.buffer_length = n;
            input_buffer.input_length = n - 1;
        }
        Err(error) => println!("error: {}", error),
    }
}
