#[cfg(test)]
mod test {
    use rustlycss_parser::parser::Parser;
    use rustlycss_codegen::Generator;
    use rustlycss_nested::NestedVisitor;
    use rustlycss_types::config::GeneralConfig;
    use std::fs::{File, read_to_string};
    use std::io::Write;

    macro_rules! test_case {
        ($path: expr ) => {
            let scss_file_path = format!("{}.scss", $path);
            let css_file_path = format!("{}.css", $path);
            let config = GeneralConfig::from(false, false);
            let code =  match read_to_string(scss_file_path.as_str()) {
                Ok(file) => {
                    file
                }
                Err(reason) => {
                    panic!("failed to open scss test file. {}", reason);
                }
            };
            let mut parser = Parser::new(code.as_str(), &config);
            let mut root = parser.parse();
            let mut codegen = Generator::new(&config);
            let mut visitor = NestedVisitor::new();
            visitor.visit(&mut root);
            codegen.generate(&mut root);
    
            match read_to_string(css_file_path.as_str()) {
                Ok(file) => {
                    assert_eq!(codegen.output, file);
                }
                Err(_reason) => {
                    let mut output = File::create(css_file_path.as_str()).unwrap();
                    match write!(output, "{}", codegen.output.as_str()) {
                        Ok(_) => {},
                        Err(_) => {}
                    }
                }
            };
        };
    }
    #[test]
    fn test_nested_double_sign_rule() {
        test_case!("./tests/fixtures/double-sign-rule");
    }
    #[test]
    fn test_nested_media_nested_rule() {
        test_case!("./tests/fixtures/media-nested-rule");
    }
    #[test]
    fn test_nested_rule_nested_rule() {
        test_case!("./tests/fixtures/rule-nested-rule");
    }
    #[test]
    fn test_nsted_sign_after_some_selector() {
        test_case!("./tests/fixtures/sign-after-some-selector");
    }
}