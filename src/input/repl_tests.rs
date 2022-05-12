#[cfg(test)]
mod repl_tests {
    use super::super::*;
    use crate::repl::MetaCommandResult::MetaCommandExit;
    use crate::InputBuffer;

    #[test]
    fn do_meta_command_test() {
        let test1 = do_meta_command(&String::from(".exit"));
        assert_eq!(test1, MetaCommandExit);
    }

    #[test]
    fn read_input_test() {
        let mut input_buffer: InputBuffer = InputBuffer::create_empty_input_buffer();
        let input = b"Testing Input Buffer\n";
        read_input(&mut input_buffer, &input[..]);
        assert_eq!(input_buffer.buffer, String::from("Testing Input Buffer"));
        assert_eq!(input_buffer.buffer_length, 21);
        assert_eq!(input_buffer.input_length, 20);
    }
}
