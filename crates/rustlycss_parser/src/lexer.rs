use rustlycss_types::position::Position;
use rustlycss_types::config::GeneralConfig;
use rustlycss_types::token::*;
use std::borrow::BorrowMut;
use std::str::CharIndices;
use crate::syntax_error;

pub struct Lexer<'a> {

    source: &'a str,
    _config: &'a GeneralConfig,

    iter: CharIndices<'a>,
    iter_char: Option<char>,
    iter_byte_index: usize,

    start_byte_index: usize,
    finish_byte_index: usize,

    token: Token,

    pos: Position,
    start_pos: Position,
    finish_pos: Position,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, config: &'a GeneralConfig) -> Self {
        let mut iter = source.char_indices();
        let frist_tuple = iter.borrow_mut().next();
        Lexer { 
            source, 
            _config: config,
            iter, 
            iter_char: Some(frist_tuple.as_ref().unwrap().1), 
            iter_byte_index: frist_tuple.unwrap().0, 
            start_byte_index: 0,
            finish_byte_index: 0,
            token: Token::Start, 
            pos: Position::new(),
            start_pos: Position::new(), 
            finish_pos: Position::new(),
        }
    }
    #[inline]
    fn get_char(&self) -> Option<char>{
        return self.iter_char;
    }
    #[inline]
    pub fn eat_char(&mut self, mut n: usize) {
        loop {
            if n == 0 {
                break;
            }
            match self.get_char() {
                None => {
                    break;
                }
                Some(code) => {
                    /*
                       TODO: performance impact
                     */
                   // if self._config.sourcemap {
                    match code {
                        '\n' => {
                            self.pos.col = 0;
                            self.pos.row += 1; 
                        }
                        _ => {
                            self.pos.col += 1;
                        }
                    }
                   // }
                    n -= 1;
                    match self.iter.next() {
                        Some(tuple) => {
                            self.iter_char = Some(tuple.1);
                            self.iter_byte_index = tuple.0;
                        }
                        None => {
                            self.iter_char = None; 
                            self.iter_byte_index = self.source.len();
                        }
                    }
                }
            }
        }
    }
    #[inline]
    pub fn start_with(&mut self, pat: &str) -> bool {
        self.source[self.iter_byte_index..].starts_with(pat)
    }
    #[inline]
    fn start_token(&mut self) {
        self.start_byte_index = self.iter_byte_index;
       // if self._config.sourcemap {
        self.start_pos = self.pos.clone();
       // }
    }
    #[inline]
    fn finish_token(&mut self) {
        self.finish_byte_index = self.iter_byte_index;
      //  if self._config.sourcemap {
        self.finish_pos = self.pos.clone();
      //  }
    }
    pub fn get_start_pos(&self)-> Position {
        self.start_pos.clone()
    }
    pub fn get_finish_pos(&self) -> Position {
        self.finish_pos.clone()
    }
    pub fn get_start_byte_index(&self) -> usize {
        self.start_byte_index
    }
    pub fn get_finish_byte_index(&self) -> usize {
        self.finish_byte_index
    }

    pub fn get_sub_str(&self, start_index: usize, end_index:usize) -> &'a str {
        &self.source[start_index..end_index]
    }
    pub fn get_token(&self) -> Token {
        self.token.clone()
    }
    pub fn next_token(&mut self)-> Token {
        self.start_token();
        self.token = match self.get_char() {
            None => {
                Token::EOF
            }
            Some(code) => {
                match code {
                    AT_CHAR => {
                        self.eat_char(1);
                        self.finish_token();
                        Token::At
                    }
                    SEMI_CHAR => {
                        self.eat_char(1);
                        self.finish_token();
                        Token::Semi
                    }
                    COLON_CHAR => {
                        self.eat_char(1);
                        self.finish_token();
                        Token::Colon
                    }
                    COMMA_CHAR => {
                        self.eat_char(1);
                        self.finish_token();
                        Token::Comma
                    }
                    BRACES_LEFT_CHAR => {
                        self.eat_char(1);
                        self.finish_token();
                        Token::BracesLeft
                    }
                    BRACES_RIGHT_CHAR => {
                        self.eat_char(1);
                        self.finish_token();
                        Token::BracesRight
                    }
                    BRACKET_LEFT_CHAR => {
                        self.eat_char(1);
                        self.finish_token();
                        Token::BracketLeft
                    }
                    BRACKET_RIGHT_CHAR => {
                        self.eat_char(1);
                        self.finish_token();
                        Token::BracketRight
                    }
                    PARENTHESES_LEFT_CHAR => {
                        self.eat_char(1);
                        self.finish_token();
                        Token::ParenthesesLeft
                    }
                    PARENTHESES_RIGHT_CHAR => {
                        self.eat_char(1);
                        self.finish_token();
                        Token::ParenthesesRight
                    }
                    SPACE_CHAR | TAB_CHAR => {
                        self.eat_char(1);
                        self.finish_token();
                        Token::Space
                    }
                    NEWLINE_CHAR => {
                        self.eat_char(1);
                        self.finish_token();
                        Token::NewLine
                    }
                    SINGLE_QUOTE | DOUBLE_QUOTE => {
                        self.read_string_literal()
                    }
                    _ => {
                        if self.start_with("/*") {
                            self.read_comment()
                        }else {
                            self.read_word()
                        }
                    }
                }
            }
        };
        return self.token.clone();
    }
    #[inline]
    fn read_string_literal(&mut self) -> Token{
        let close_char =  match self.get_char() {
            None => panic!("[Internal Error]: string literal should be start with quote, got None"),
            Some(code) => {
                match code {
                    SINGLE_QUOTE => SINGLE_QUOTE,
                    DOUBLE_QUOTE => DOUBLE_QUOTE,
                    _ => panic!("[Internal Error]: string literal should be start with quote,")
                }
            }
        };
        self.eat_char(1);
        loop {
            match self.get_char() {
                None => {
                    self.finish_token();
                    syntax_error!(format!("Unclose string Literal, expect close char {:?}", close_char));
                }
                Some(code) => {
                    self.eat_char(1);
                    if code == close_char {
                        self.finish_token();
                        return Token::StringLiteral
                    }
                }
            } 
        };
    }
    #[inline]
    fn read_word(&mut self) -> Token {
        loop {
            match self.get_char() {
                None => {
                    self.finish_token();
                    return Token::EOF;
                }
                Some(code)=> {
                    match code {
                        AT_CHAR |
                        SEMI_CHAR |
                        COLON_CHAR |
                        COMMA_CHAR |
                        BRACES_LEFT_CHAR |
                        BRACES_RIGHT_CHAR |
                        BRACKET_LEFT_CHAR |
                        BRACKET_RIGHT_CHAR |
                        PARENTHESES_LEFT_CHAR |
                        PARENTHESES_RIGHT_CHAR |
                        SPACE_CHAR | TAB_CHAR |
                        NEWLINE_CHAR |
                        SINGLE_QUOTE | DOUBLE_QUOTE => {
                            self.finish_token();
                            return Token::Word;
                        }
                        _ => {
                            self.eat_char(1)
                        }
                    }
                }
            }
        }
    }
    #[inline]
    fn read_comment(&mut self) -> Token {
        if !self.start_with("/*") {
            panic!("[Internal Error]: read comment function should start with `/*`");
        }
        self.eat_char(2);
        loop {
            match self.get_char() {
                None => {
                    syntax_error!(format!("Unclose comment, start position {}", self.get_start_byte_index()));
                }
                Some(_code)=> {
                    if self.start_with("*/") {
                        self.eat_char(2);
                        self.finish_token();
                        return Token::Comment;
                    }
                    self.eat_char(1)
                }
            }
        }
    }
}