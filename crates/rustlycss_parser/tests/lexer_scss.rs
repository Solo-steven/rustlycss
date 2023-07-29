#[cfg(test)]
mod test {
    use rustlycss_parser::lexer::Lexer;
    use rustlycss_types::token::Token;
    use std::fs::{File, read_to_string};
    use std::io::Write;

    macro_rules! test_case {
        ($path: expr ) => {
            let css_file_path = format!("{}.scss", $path);
            let token_file_path = format!("{}.token.txt", $path);
            let code =  match read_to_string(css_file_path.as_str()) {
                Ok(file) => {
                    file
                }
                Err(reason) => {
                    panic!("failed to open scss test file. {}", reason);
                }
            };
            let mut lexer = Lexer::new(code.as_str());
            let mut token_string = String::new();
            loop {
                let t = lexer.next_token();
                let start = lexer.get_start_byte_index();
                let finish = lexer.get_finish_byte_index();
                let value = lexer.get_sub_str(start, finish);
                match t {
                    Token::EOF => {
                        token_string.push_str(format!("[TOKEN: {:?}, value: {:?}, start: {:?}, finish: {:?}]\n", t,value, start, finish ).as_str());
                        break;
                    }
                    _ => {
                        token_string.push_str(format!("[TOKEN: {:?}, value: {:?}, start: {:?}, finish: {:?}]\n", t,value, start, finish ).as_str());
                    }
                }
            }
    
            match read_to_string(token_file_path.as_str()) {
                Ok(file) => {
                    assert_eq!(token_string, file);
                }
                Err(_reason) => {
                    let mut output = File::create(token_file_path.as_str()).unwrap();
                    match write!(output, "{}", token_string.as_str()) {
                        Ok(_) => {},
                        Err(_) => {}
                    }
                }
            };
        };
    }
    #[test]
    fn test_scss_nested_classes() {
        test_case!("./tests/fixtures/scss/nested-classes");
    }
    #[test]
    fn test_scss_nested_pseudo_classes() {
        test_case!("./tests/fixtures/scss/nested-pseudo-classes");
    }
    #[test]
    fn test_scss_nested_pseudo_element() {
        test_case!("./tests/fixtures/scss/nested-pseudo-element");
    }
    #[test]
    fn test_scss_nested_media_under_atrule() {
        test_case!("./tests/fixtures/scss/nested-media-under-atrule");
    }
    #[test]
    fn test_scss_nested_media_under_rule() {
        test_case!("./tests/fixtures/scss/nested-media-under-rule");
    }
    #[test]
    fn test_scss_nested_wrapped_by_rule() {
        test_case!("./tests/fixtures/scss/nested-media-wrapped-by-rule");
    }


}