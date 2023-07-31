use rustlycss_parser::parser::Parser;
use rustlycss_types::config::GeneralConfig;

fn main() {
    let code = "
        .test {
            color: red;
        }
    ";
    let config = GeneralConfig::from(true, true);
    let mut parser = Parser::new(code, &config);
    let root = parser.parse();
    println!("{:?}", root);
}