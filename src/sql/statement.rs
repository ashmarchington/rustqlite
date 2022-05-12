use crate::input;
use crate::sql::column;

pub enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}

pub enum StatementType {
    StatementInsert,
    StatementSelect,
    StatementInitial,
}

pub struct Statement {
    statement_type: StatementType,
}

impl Statement {
    pub fn new(statement: StatementType) -> Statement {
        Statement {
            statement_type: statement,
        }
    }

    pub fn set_type(&mut self, statement: StatementType) {
        self.statement_type = statement;
    }

    pub fn get_type(self) -> StatementType {
        self.statement_type
    }
}

pub fn prepare_statement(
    input_buffer: &input::InputBuffer,
    statement: &mut Statement,
) -> PrepareResult {
    if input_buffer.buffer.starts_with("insert") {
        statement.set_type(StatementType::StatementInsert);

        return PrepareResult::PrepareSuccess;
    }
    if input_buffer.buffer.starts_with("select") {
        statement.set_type(StatementType::StatementSelect);

        return PrepareResult::PrepareSuccess;
    }

    PrepareResult::PrepareUnrecognizedStatement
}

pub fn execute_statement(statement: Statement) -> () {
    match statement.get_type() {
        StatementType::StatementInsert => {
            println!("This would run an insert statement.\n")
        }
        StatementType::StatementSelect => {
            println!("This would run a select statement.\n")
        }
        _ => {}
    }
}
