mod error;
mod input;
mod sql;
mod utility;

use crate::input::InputBuffer;
use crate::sql::column::definition::Varchar;
use input::repl;
use sql::statement;
use std::io;
use std::process::exit;

// Standard exit success
const EXIT_SUCCESS: i32 = 0;

// Main driver
fn main() {
    let mut input_buffer: InputBuffer = InputBuffer::create_empty_input_buffer();

    loop {
        repl::print_prompt();
        let stdio = io::stdin();
        let input = stdio.lock();
        repl::read_input(&mut input_buffer, input);
        let command = &input_buffer.buffer;
        if command.find('.').unwrap_or(1) == 0 {
            match repl::do_meta_command(command) {
                repl::MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!("Unrecognized command: {}.\n", command);
                    continue;
                }
                repl::MetaCommandResult::MetaCommandSuccess => {
                    continue;
                }
                repl::MetaCommandResult::MetaCommandExit => {
                    exit(EXIT_SUCCESS);
                }
            }
        }

        let mut statement: statement::Statement =
            statement::Statement::new(statement::StatementType::StatementInitial);
        match statement::prepare_statement(&input_buffer, &mut statement) {
            statement::PrepareResult::PrepareSuccess => {
                println!("Statement prepared correctly");
            }
            statement::PrepareResult::PrepareUnrecognizedStatement => {
                println!(
                    "Unrecognized keyword ar start of {}.\n",
                    input_buffer.buffer
                );
                continue;
            }
        }
        statement::execute_statement(statement);
        println!("Executed.\n")
    }
}
