pub mod repl;

// Holds data that has been pulled from an input line
pub struct InputBuffer {
    pub buffer: String,
    pub buffer_length: usize,
    pub input_length: usize,
}

// Create an empty InputBuffer
pub fn new_input_buffer() -> InputBuffer {
    let input_buffer = InputBuffer {
        buffer: String::new(),
        buffer_length: 0,
        input_length: 0,
    };

    input_buffer
}
