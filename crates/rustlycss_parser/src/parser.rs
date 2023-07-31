use rustlycss_types::token::Token;
use rustlycss_types::position::{Span, Position, Location};
use rustlycss_types::config::GeneralConfig;
use rustlycss_types::ast::*;
use std::borrow::Cow;
use crate::syntax_error;
use crate::lexer::Lexer;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    _config: &'a GeneralConfig,
}

impl<'a> Parser <'a> {
    pub fn new(code:&'a str, config: &'a GeneralConfig) -> Self {
        Parser {
            lexer: Lexer::new(code, config),
            _config: config
        }
    }
    // compsition method for lexer
    #[inline]
    pub fn next_token(&mut self,) -> Token {
        let mut token = self.lexer.next_token();
        loop {
            match token {
                Token::Comment => token = self.lexer.next_token(),
                _ => return token
            }
        }
    }
    // composition method for lexer
    #[inline]
    pub fn get_token(&mut self) -> Token {
        let mut token = self.lexer.get_token();
        loop {
            match token {
                Token::Comment | Token::Start => token = self.lexer.next_token(),
                _ => return token,
            }
        }
    }
    // composition method for lexer
    #[inline]
    fn get_start_byte_index(&self) -> usize {
        self.lexer.get_start_byte_index()
    }
    // composition method for lexer
    #[inline]
    fn get_finish_byte_index(&self) -> usize {
        self.lexer.get_finish_byte_index()
    }
    fn get_start_pos(&self) -> Position {
        self.lexer.get_start_pos()
    }
    fn get_finish_pos(&self) -> Position {
        self.lexer.get_finish_pos()
    }
    #[inline]
    fn skip_changeline_and_space(& mut self) {
        loop {
            match self.get_token() {
                Token::NewLine | Token::Space  => self.next_token(),
                _ => break 
            };
        }
    }
    #[inline]
    pub fn parse(&mut self,) -> Root<'a> {
        self.parse_root()
    }
    #[inline]
    fn parse_root(&mut self) -> Root<'a> {
        let mut nodes = Vec::<Child>::new();
        let start_pos = self.get_start_pos();
        loop {
            self.skip_changeline_and_space();
            match self.get_token() {
                Token::EOF => {
                    break;
                }
                Token::At => {
                    nodes.push(Child::AtRule(self.parse_at_rule()))
                }
                Token::Semi => {
                    self.next_token();
                }
                _ => {
                    nodes.push(self.parse_declar_or_rule());
                }
            }
        }
        let finish_byte_index = self.get_finish_byte_index();
        let finish_pos = self.get_finish_pos();
        return Root { nodes, span: Span::from(0 , finish_byte_index), loc: Location::from(start_pos, finish_pos) }
    }
    fn parse_at_rule(&mut self) -> AtRule<'a> {
        let start_byte_index: usize;
        let finish_byte_index: usize;
        let start_pos: Position;
        let finish_pos: Position;
        match self.get_token() {
            Token::At => {
                start_byte_index = self.get_start_byte_index();
                start_pos = self.get_start_pos();
                self.next_token();
            }
            _ => {
                syntax_error!("at rule must start with @ char");
            }
        }
        let name = self.parse_at_rule_name();
        self.skip_changeline_and_space();
        let mut param: Option<Cow<'a, str>> = None;
        let mut nodes: Option<Vec<Child<'_>>> = None;
        match self.get_token() {
            Token::Semi => {
                finish_byte_index = self.get_finish_byte_index();
                finish_pos = self.get_finish_pos();
                self.next_token();
                return AtRule { 
                    name, param, nodes, 
                    span: Span::from(start_byte_index, finish_byte_index),
                    loc: Location::from(start_pos, finish_pos)
                };
            }
            Token::BracesLeft => {/* fullover */}
            _ => {
                param = Some(self.parse_at_rule_param());
            }
        }
        match self.get_token() {
            Token::BracesLeft => nodes = Some(self.parse_nodes_in_braces()),
            _ => nodes = None,
        }
        finish_byte_index = self.get_start_byte_index();
        finish_pos = self.get_finish_pos();
        return AtRule { 
            name, param, nodes, 
            span: Span::from(start_byte_index, finish_byte_index),
            loc: Location::from(start_pos, finish_pos)
        };
    }
    #[inline]
    fn parse_at_rule_name(&mut self) -> Cow<'a, str> {
        let start_index_of_name = self.get_start_byte_index();
        let mut end_index_of_name : usize  = self.get_start_byte_index();
        loop {
            match self.get_token() {
                Token::EOF |
                Token::Space | Token::NewLine | Token::Semi |
                Token::BracesLeft | Token::BracesRight |
                Token::BracketLeft | Token::BracketRight |
                Token::ParenthesesLeft | Token::ParenthesesRight => {
                    break;
                }
                _ => {
                    end_index_of_name = self.get_finish_byte_index();
                    self.next_token();
                }
            }
        }
        return Cow::Borrowed(self.lexer.get_sub_str(start_index_of_name, end_index_of_name));
    }
    #[inline]
    fn parse_at_rule_param(&mut self) -> Cow<'a, str> {
        let start_index_of_param = self.get_start_byte_index();
        let mut end_index_of_param : usize  = self.get_finish_byte_index();
        loop {
            match self.get_token() {
                Token::BracesLeft | Token::BracesRight => {
                    break;
                }
                Token::Semi => {
                    self.next_token();
                    break;
                }
                Token::NewLine | Token::Space => {
                    self.next_token();
                }
                _ => {
                    end_index_of_param = self.get_finish_byte_index();
                    self.next_token();
                }
            }
        }
        return Cow::Borrowed(self.lexer.get_sub_str(start_index_of_param, end_index_of_param));
    }

    fn parse_nodes_in_braces(&mut self) -> Vec<Child<'a>> {
        match self.get_token() {
            Token::BracesLeft => {
                self.next_token();
            }
            _ => {
                syntax_error!("nodes must wrap in braces");
            }
        }
        let mut nodes = Vec::<Child>::new();
        loop {
            self.skip_changeline_and_space();
            let token = self.get_token();
            match token {
                Token::EOF | Token::BracesRight => {
                    break;
                }
                Token::Semi => {
                    self.next_token();
                }
                Token::At => {
                    nodes.push(Child::AtRule(self.parse_at_rule()))
                }
                _ => {
                    nodes.push(self.parse_declar_or_rule());
                }
            }
        }
        match self.get_token() {
            Token::BracesRight => {
                self.next_token();
            }
            _ => {
                syntax_error!("nodes must wrap in braces");
            }
        }
        return nodes;
    }
    // this function only call in `parse_nodes_in_braces` and `parse_root` parse loop
    // after `skip_changeline_and_space`, so frist token in this function main loop 
    // is not change and space
    #[inline]
    fn parse_declar_or_rule(&mut self) -> Child<'a> {
        let start_pos = self.get_start_pos();
        let start_index_of_name = self.get_start_byte_index();
        let mut end_index_of_name: usize = self.get_finish_byte_index();
        let mut is_space_or_changeline_between = false;
        loop {
            match self.get_token() {
                Token::Colon  => {
                    return self.parse_start_with_colon(
                        start_index_of_name,
                        end_index_of_name,
                        start_pos,
                        is_space_or_changeline_between, 
                    );
                }
                Token::BracesLeft => {
                    return Child::Rule(self.parse_rule_with_selector(
                        Cow::Borrowed(self.lexer.get_sub_str(start_index_of_name,end_index_of_name)),
                        start_index_of_name,
                        start_pos,
                    ));
                }
                Token::NewLine | Token::Space => {
                    is_space_or_changeline_between = true;
                    self.next_token();
                }
                _ => {

                    end_index_of_name = self.get_finish_byte_index();
                    self.next_token();
                }
            }
        }
    }
    // start with colon maybe is have below three condition occur. with two format
    // frist  format <>:<>;, end with semi, have to be a declaration
    // second format <>:<>{, end with BracesLeft, have to be a rule.
    // 1. selector start with colon: `:root`
    // 2. selector have colon: `.some-class:hover`
    // 3. declaration: `color: blue`
    // so this function loop with end when meet BracesLeft or semi.
    fn parse_start_with_colon(
        &mut self, 
        start_index_of_prop_or_selector :usize , 
        end_index_of_prop_or_selector: usize, 
        start_pos: Position,
        is_space_or_newline_between: bool
    ) -> Child<'a> {
        // should start with colon
        match self.get_token() {
            Token::Colon => self.next_token(),
            _ => panic!("[Internal Error]: parse_start_with_colon_should start with colon token.")
        };
        // start function loop with any char after colon, so may be changeline and space, so 
        // we need a flag to determinate when to init start_byte_index 's value.
        let mut is_start_not_changeline_and_space = false;
        let mut start_index_of_value: usize = self.get_start_byte_index();
        let mut end_index_of_value_or_selector: usize = self.get_start_byte_index();
        loop {
            let token = self.get_token();
            match token {
                Token::BracesLeft  => {
                    return Child::Rule(self.parse_rule_with_selector( 
                        Cow::Borrowed(self.lexer.get_sub_str(
                            start_index_of_prop_or_selector,
                            end_index_of_value_or_selector,
                        )),
                        start_index_of_prop_or_selector,
                        start_pos,
                    ));
                }
                Token::Semi | Token::BracesRight => {
                    if token == Token::Semi {
                        self.next_token();
                    }
                    if is_space_or_newline_between {
                        syntax_error!("Declaration prop can not have space or new line");
                    }
                    return Child::Declar(Declaration { 
                        prop: Cow::Borrowed(self.lexer.get_sub_str(start_index_of_prop_or_selector,end_index_of_prop_or_selector)),
                        value: Cow::Borrowed(self.lexer.get_sub_str(start_index_of_value, end_index_of_value_or_selector)),
                        span: Span::from(start_index_of_prop_or_selector, self.get_start_byte_index()),
                        loc: Location::from(start_pos, self.get_finish_pos())
                    });
                }
                Token::NewLine | Token::Space => {
                    self.next_token();
                }
                _ => {
                    if is_start_not_changeline_and_space == false {
                        is_start_not_changeline_and_space = true;
                        start_index_of_value = self.get_start_byte_index();
                    }
                    end_index_of_value_or_selector = self.get_finish_byte_index();
                    self.next_token();
                }
            }
        }
        // parse declaration end
    }
    #[inline]
    fn parse_rule_with_selector(&mut self, selector: Cow<'a, str>, start_byte_index: usize, start_pos: Position) -> Rule<'a> {
        match self.get_token() {
            Token::BracesLeft => {
                let nodes = self.parse_nodes_in_braces();
                Rule { 
                    selector,
                    nodes, 
                    span: Span::from(start_byte_index, self.get_start_byte_index()),
                    loc: Location::from(start_pos, self.get_finish_pos())
                 }
            }
            _ => {
                syntax_error!("rule must have brace");
            }
        }
    }
}