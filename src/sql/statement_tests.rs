#[cfg(test)]
mod statement_tests {
    use crate::statement::PrepareResult::{PrepareSuccess, PrepareUnrecognizedStatement};
    use crate::statement::StatementType::StatementInitial;
    use crate::statement::{prepare_statement, Statement};
    use crate::InputBuffer;

    #[test]
    fn prepare_statement_test() {
        let select = InputBuffer::new(String::from("select * from test;"), 99, 99);
        let insert = InputBuffer::new(String::from("insert into test;"), 99, 99);
        let unknown = InputBuffer::new(String::from("unknown"), 99, 99);
        let mut select_statement = Statement::new(StatementInitial);
        let mut insert_statement = Statement::new(StatementInitial);
        let mut unknown_statement = Statement::new(StatementInitial);
        let select_test = prepare_statement(&select, &mut select_statement);
        assert_eq!(select_test, PrepareSuccess);
        let insert_test = prepare_statement(&insert, &mut insert_statement);
        assert_eq!(insert_test, PrepareSuccess);
        let unknown_test = prepare_statement(&unknown, &mut unknown_statement);
        assert_eq!(unknown_test, PrepareUnrecognizedStatement);
    }
}
