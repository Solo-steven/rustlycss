pub mod lexer;
pub mod parser;
mod marco;

use rustlycss_types::token::Token;
use rustlycss_types::config::GeneralConfig;
use rustlycss_types::visitor::{Visitor, Walkable};
use rustlycss_parser::lexer::Lexer;
use rustlycss_parser::parser::Parser;

const TINT_FILE_STR: &str = include_str!("../../../assets/bootstrap-rebot.css");
const BIG_FILE_STR: &str =  include_str!("../../../assets/bootstrap.css");
const HUGE_FILE_STR: &str = include_str!("../../../assets/tailwind-dark.css");
const TINR_SCSS_FILE: &str = include_str!("../../../assets/nested-tiny.scss");


fn to_tokens(code: &str) {
    let config = GeneralConfig::from(true, false);
    let mut lexer = Lexer::new(code, &config);
    loop {
        let t = lexer.next_token();
        match t {
            Token::EOF => {
                println!("[TOKEN: {:?}, value: {:?}]",t, lexer.get_sub_str(lexer.get_start_byte_index(), lexer.get_finish_byte_index()));
                break;
            }
            _ => {
                println!("[TOKEN: {:?}, value: {:?}]", t, lexer.get_sub_str(lexer.get_start_byte_index(), lexer.get_finish_byte_index()));
            }
        }
    }
}

fn main() {
    let config = GeneralConfig::from(true, false);
    let code = r#"
        .container {
            color: red;
        }
    "#;
    to_tokens(code);
    let mut parser = Parser::new(code, &config);
    let root = parser.parse();
    println!("{:?}", root);
}