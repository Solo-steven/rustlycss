#[cfg(test)]
mod test {
    use rustlycss_parser::lexer::Lexer;
    use rustlycss_types::token::Token;
    use std::fs::{File, read_to_string};
    use std::io::Write;

    macro_rules! test_case {
        ($path: expr ) => {
            let css_file_path = format!("{}.css", $path);
            let token_file_path = format!("{}.token.txt", $path);
            let code =  match read_to_string(css_file_path.as_str()) {
                Ok(file) => {
                    file
                }
                Err(reason) => {
                    panic!("failed to open css test file. {}", reason);
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
    fn test_base_line_atrule_keyframes() {
        test_case!("./tests/fixtures/baseline/atrule-keyframes");
    }
    #[test]
    fn test_base_line_atrule_no_param() {
        test_case!("./tests/fixtures/baseline/atrule-no-param");
    }
    #[test]
    fn test_base_line_atrule_simple() {
        test_case!("./tests/fixtures/baseline/atrule-simple");
    }
    #[test]
    fn test_base_line_atrule_skip_param() {
        test_case!("./tests/fixtures/baseline/atrule-skip-body");
    }
    #[test]
    fn test_base_line_rule_colon_start() {
        test_case!("./tests/fixtures/baseline/rule-colon-start");
    }
    #[test]
    fn test_base_line_pseudo_classes() {
        test_case!("./tests/fixtures/baseline/rule-pseudo-classes");
    }
    #[test]
    fn test_base_line_pseudo_elements() {
        test_case!("./tests/fixtures/baseline/rule-pseudo-element");
    }
    #[test]
    fn test_base_line_utf8_string() {
        test_case!("./tests/fixtures/baseline/utf8-string");
    }
    #[test]
    fn test_base_line_rule_selector_adjacent() {
        test_case!("./tests/fixtures/baseline/rule-selector-adjacent");
    }
    #[test]
    fn test_base_line_rule_selector_child() {
        test_case!("./tests/fixtures/baseline/rule-selector-child");
    }
    #[test]
    fn test_base_line_rule_selector_descendant() {
        test_case!("./tests/fixtures/baseline/rule-selector-descendant");
    }
    #[test]
    fn test_base_line_rule_selector_sibling() {
        test_case!("./tests/fixtures/baseline/rule-selector-sibling");
    }
}