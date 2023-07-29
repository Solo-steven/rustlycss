pub mod lexer;
pub mod parser;

use rustlycss_types::token::Token;
use rustlycss_types::visitor::{Visitor, Walkable};
use rustlycss_parser::lexer::Lexer;
use rustlycss_parser::parser::Parser;

const TINT_FILE_STR: &str = include_str!("../../../assets/bootstrap-rebot.css");
const BIG_FILE_STR: &str =  include_str!("../../../assets/bootstrap.css");
const HUGE_FILE_STR: &str = include_str!("../../../assets/tailwind-dark.css");
const TINR_SCSS_FILE: &str = include_str!("../../../assets/nested-tiny.scss");


fn to_tokens(code: &str) {
    let mut lexer = Lexer::new(code);
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
    let code = r#"
        .container {
            @media screen and (max-width: 1280px) {
                height: 100px;
                width: 200px
            }
            & > .item {
                @media screen and (max-width) {
                    height: 400px;
                }
            }
        }
    "#;
    // let mut lexer = Lexer::new(BIG_FILE_STR);
    to_tokens(code);
    let mut parser = Parser::new(code);
    let root = parser.parse();
    println!("{:?}", root);
}