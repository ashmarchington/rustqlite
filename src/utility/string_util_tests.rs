#[cfg(test)]
mod string_util_tests {
    use crate::utility::string_util::remove_newline;

    #[test]
    fn remove_newline_test() -> () {
        let test1 = String::from("remove this newline ->\n");
        let removed = remove_newline(test1);
        assert_eq!(removed, "remove this newline ->");
        let test2 = String::from("remove this newline and carriage ->\r\n");
        let removed2 = remove_newline(test2);
        assert_eq!(removed2, "remove this newline and carriage ->");
    }
}
