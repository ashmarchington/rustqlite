#[cfg(test)]
#[path = "string_util_tests.rs"]
mod string_util_tests;

// Removes a new line character from the end of a string
pub fn remove_newline(mut s: String) -> String {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }

    s
}
