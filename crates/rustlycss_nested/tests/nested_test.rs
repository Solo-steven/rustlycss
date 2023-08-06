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
            let scss_file_path = format!("{}.postcss", $path);
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
    fn test_bem_naming() {
        test_case!("./tests/fixtures/bem-naming");
    }
    #[test]
    fn test_correctly_replace_tail_ampersands_that_are_futher_down() {
        test_case!("./tests/fixtures/correctly-replaces-tail-ampersands-that-are-nested-further-down");
    }
    #[test]
    fn test_correctly_replaces_tail_ampersands_that_are_nested_inside_ampersand_rules() {
        test_case!("./tests/fixtures/correctly-replaces-tail-ampersands-that-are-nested-inside-ampersand-rules");
    }
    #[test]
    fn test_correctly_replaces_tail_ampersands() {
        test_case!("./tests/fixtures/correctly-replaces-tail-ampersands");
    }
    #[test]
    fn test_does_not_replace_ampersand_inside_string() {
        test_case!("./tests/fixtures/does-not-replace-ampersand-inside-string");
    }
    #[test]
    fn test_double_bem_naming() {
        test_case!("./tests/fixtures/double-bem-naming");
    }
    #[test]
    fn test_group_rule_for_declarations_after_nested_rule() {
        test_case!("./tests/fixtures/group-rule-for-declarations-after-nested-rule");
    }
    #[test]
    fn test_handles_host_selector_case() {
        test_case!("./tests/fixtures/handles-host-selector-case");
    }
    #[test]
    fn test_leaves_nested_media_blocks_as_is() {
        test_case!("./tests/fixtures/leaves-nested-media-blocks-as-is");
    }
    #[test]
    fn test_multi_nested_media_is_resolved() {
        test_case!("./tests/fixtures/multi-nested-media-is-resolved");
    }
    #[test]
    fn test_process_comma() {
        test_case!("./tests/fixtures/process-comma");
    }
    #[test]
    fn test_processes_comma_inside() {
        test_case!("./tests/fixtures/processes-comma-inside");
    }
    #[test]
    fn test_processes_comma_with_ampersand() {
        test_case!("./tests/fixtures/processes-comma-with-ampersand");
    }
    #[test]
    fn test_replace_ampersands() {
        test_case!("./tests/fixtures/replace-ampersands");
    }
    #[test]
    fn test_replaces_ampersand_in_adjacent_sibling_selector() {
        test_case!("./tests/fixtures/replaces-ampersand-in-adjacent-sibling-selector");
    }
    #[test]
    fn test_replaces_ampersand() {
        test_case!("./tests/fixtures/replaces-ampersand");
    }
    #[test]
    fn test_replaces_ampersands_in_not_selector() {
        test_case!("./tests/fixtures/replaces-ampersands-in-not-selector");
    }
    #[test]
    fn test_unwraps_atrule_with_rules() {
        test_case!("./tests/fixtures/unwraps-atrule-with-rules");
    }
    #[test]
    fn test_unwraps_atrule() {
        test_case!("./tests/fixtures/unwraps-atrule");
    }
    #[test]
    fn test_unwraps_atrules_with_interleaved_properties() {
        test_case!("./tests/fixtures/unwraps-atrules-with-interleaved-properties");
    }
    #[test]
    fn test_unwraps_atrules() {
        test_case!("./tests/fixtures/unwraps-atrules");
    }
    #[test]
    fn test_unwraps_font_face_to_top_level_css() {
        test_case!("./tests/fixtures/unwraps-font-face-to-top-level-css");
    }
    #[test]
    fn test_unwraps_keyframes() {
        test_case!("./tests/fixtures/unwraps-keyframes");
    }
    #[test]
    fn test_unwraps_multiple_font_face_to_top_level_css() {
        test_case!("./tests/fixtures/unwraps-multiple-fonts-to-top-level-css");
    }
    #[test]
    fn test_unwraps_rule_inside_atrule() {
        test_case!("./tests/fixtures/unwraps-rule-inside-atrule");
    }
    #[test]
    fn test_does_not_move_custom_atrules_placed_under_nested() {
        test_case!("./tests/fixtures/does-not-move-custom-atrules-placed-under-nested");
    } 
}