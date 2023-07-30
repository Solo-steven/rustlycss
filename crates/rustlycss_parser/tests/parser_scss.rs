#[cfg(test)]
mod test {
    use rustlycss_parser::parser::Parser;
    use rustlycss_types::config::GeneralConfig;
    use std::fs::{File, read_to_string};
    use std::io::Write;
    use serde_json::to_string_pretty;

    macro_rules! test_case {
        ($path: expr ) => {
            let css_file_path = format!("{}.scss", $path);
            let ast_file_path = format!("{}.ast.json", $path);
            let config = GeneralConfig::from(true, false);
            let code =  match read_to_string(css_file_path.as_str()) {
                Ok(file) => {
                    file
                }
                Err(reason) => {
                    panic!("failed to open scss test file. {}", reason);
                }
            };
            let mut parser = Parser::new(code.as_str(), &config);
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