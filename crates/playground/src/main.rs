use rustlycss_parser::parser::Parser;
use rustlycss_nested::NestedVisitor;
use rustlycss_codegen::Generator;
use rustlycss_types::config::GeneralConfig;
use std::fs::File;
use std::io::Write;

fn main() {
    let code = "
        .one, .class {
            &:hover {
                color: red;
            }
        }
    ";
    let config = GeneralConfig::from(false, false);
    let mut parser = Parser::new(code, &config);
    let mut root = parser.parse();
    let mut visitor = NestedVisitor::new();
    visitor.visit(&mut root);
    let mut gen = Generator::new(&config);
    gen.generate(&root);
    println!("{:?}", gen.output);
    let mut output = File::create("test.css").unwrap();
    match write!(output, "{}", gen.output.as_str()) {
        Ok(_) => {},
        Err(_) => {}
    }
}