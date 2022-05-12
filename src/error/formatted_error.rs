pub struct ErrorMessage {
    description: String,
}

impl ErrorMessage {
    pub fn new(error: String) -> ErrorMessage {
        ErrorMessage { description: error }
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
