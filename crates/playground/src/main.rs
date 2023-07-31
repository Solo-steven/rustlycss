use rustlycss_parser::parser::Parser;
use rustlycss_nested::NestedVisitor;
use rustlycss_codegen::Generator;
use rustlycss_types::config::GeneralConfig;

fn main() {
    let code = "
        .wrapper {
            @media screen and (max-width: 500px) {
                color: red;
                @media screen and (max-width: 300px) {
                    .container {
                        color: red;
                    }
                }
                .item {
                    flex: 1;
                }
            }
        }
    ";
    let config = GeneralConfig::from(true, false);
    let mut parser = Parser::new(code, &config);
    let mut root = parser.parse();
    let mut visitor = NestedVisitor::new();
    visitor.visit(&mut root);
    let mut gen = Generator::new(&config);
    gen.generate(&root);
    println!("{:?}", gen.output);
}