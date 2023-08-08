use rustlycss_parser::parser::Parser;
use rustlycss_nested::NestedVisitor;
use rustlycss_simple_vars::SimpleVarVisitor;
use rustlycss_codegen::Generator;
use rustlycss_types::config::GeneralConfig;
use std::fs::File;
use std::io::Write;

fn main() {
    let code = "
        $test: red;
        $way: button;
        $a-b_10: 1;
        div { 
            color: $(test);
        }
        block__$(way) {
            $test: red;
            a: test$a-b_10kml;
        }

    ";
    let config = GeneralConfig::from(false, false);
    let mut parser = Parser::new(code, &config);
    let mut root = parser.parse();
    println!("{:?}", root);
    let mut visitor = SimpleVarVisitor::new();
    visitor.visit(&mut root);
    let mut gen = Generator::new(&config);
    gen.generate(&root);
    println!("{:?}", gen.output);
    let mut output = File::create("test.css").unwrap();
    write!(output, "{}", gen.output.as_str()).unwrap();
}