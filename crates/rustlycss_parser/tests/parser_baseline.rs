#[cfg(test)]
mod test {
    use rustlycss_parser::parser::Parser;
    use std::fs::{File, read_to_string};
    use std::io::Write;
    use serde_json::to_string_pretty;

    macro_rules! test_case {
        ($path: expr ) => {
            let css_file_path = format!("{}.css", $path);
            let ast_file_path = format!("{}.ast.json", $path);
            let code =  match read_to_string(css_file_path.as_str()) {
                Ok(file) => {
                    file
                }
                Err(reason) => {
                    panic!("failed to open css test file. {}", reason);
                }
            };
            let mut parser = Parser::new(code.as_str());
            let root = parser.parse();
            let result_ast = to_string_pretty(&root).unwrap();
    
            match read_to_string(ast_file_path.as_str()) {
                Ok(file) => {
                    assert_eq!(result_ast, file);
                }
                Err(_reason) => {
                    let mut output = File::create(ast_file_path.as_str()).unwrap();
                    match write!(output, "{}", result_ast.as_str()) {
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