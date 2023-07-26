use std::borrow::Cow;
use crate::lexer::Lexer;
use rustlycss_types::position::Position;
use rustlycss_types::token::Token;
use rustlycss_types::ast::*;

pub struct Parser<'source_str> {
    lexer: Lexer<'source_str>,
}

struct  MetaWithNodes<'a> {
    nodes: Vec<Child<'a>>,
}

impl<'source_str> Parser <'source_str> {
    pub fn new(code:&'source_str str) -> Self {
        Parser {
            lexer: Lexer::new(code),
        }
    }
    // compsition method for lexer
    #[inline]
    pub fn next_token(&mut self,) -> Token {
        let token = self.lexer.next_token();

        if token == Token::Comment {
            return self.next_token();
        }
        return token;
        // loop {
        //     match token {
        //         Token::Comment => {
        //             token = self.lexer.next_token();
        //         }
        //         _ => {
        //             return token;
        //         }
        //     }
        // }
    }
    // composition method for lexer
    #[inline]
    pub fn get_token(&mut self) -> Token {
        let token = self.lexer.get_token();
        if token == Token::Start {
            return self.next_token();
        }
        if token == Token::Comment {
            return self.next_token();
        }
        return token;
        // let mut token = self.lexer.get_token();
        // loop {
        //     match token {
        //         Token::Comment => {
        //             token = self.lexer.next_token();
        //         }
        //         _ => {
        //             return token;
        //         }
        //     }
        // }
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
    #[inline]
    fn skip_changeline_and_space(& mut self) {
        loop {
            match self.get_token() {
                Token::NewLine | Token::Space  => {
                    self.next_token();
                }
                _ => {
                    break;
                }
            };
        }
    }
    #[inline]
    pub fn parse(&mut self,) -> Root<'source_str> {
        self.parse_root()
    }
    #[inline]
    fn parse_root(&mut self) -> Root<'source_str> {
        let mut nodes = Vec::<Child>::new();
        loop {
            self.skip_changeline_and_space();
            match self.get_token() {
                Token::EOF => {
                    break;
                }
                Token::At => {
                    nodes.push(Child::AtRule(self.parse_at_rule()))
                }
                _ => {
                    nodes.push(self.parse_declar_or_rule());
                }
            }
        }
        let finish_byte_index = self.get_finish_byte_index();
        return Root { nodes, start_byte_index: 0, finish_byte_index }
    }
    fn parse_at_rule(&mut self) -> AtRule<'source_str> {
        let start_byte_index: usize;
        let finish_byte_index: usize;
        match self.get_token() {
            Token::At => {
                start_byte_index = self.get_start_byte_index();
                self.next_token();
            }
            _ => {
                panic!("at rule must start with @ char");
            }
        }
        let name = self.parse_at_rule_name();
        self.skip_changeline_and_space();
        let mut param: Option<Cow<'source_str, str>> = None;
        let mut nodes: Option<Vec<Child<'_>>> = None;
        match self.get_token() {
            Token::Semi => {
                finish_byte_index = self.get_finish_byte_index();
                self.next_token();
                return AtRule { name, param, nodes, start_byte_index, finish_byte_index };
            }
            Token::BracesLeft => {/* fullover */}
            _ => {
                param = Some(self.parse_at_rule_param());
            }
        }
        match self.get_token() {
            Token::BracesLeft => {
                nodes = Some(self.parse_nodes_in_braces());
                finish_byte_index = self.get_start_byte_index();
            }
            _ => {
                finish_byte_index = self.get_start_byte_index();
                nodes = None;
            }
        }
        return AtRule { name, param, nodes, start_byte_index, finish_byte_index};
    }
    #[inline]
    fn parse_at_rule_name(&mut self) -> Cow<'source_str, str> {
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
    fn parse_at_rule_param(&mut self) -> Cow<'source_str, str> {
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

    fn parse_nodes_in_braces(&mut self) -> Vec<Child<'source_str>> {
        match self.get_token() {
            Token::BracesLeft => {
                self.next_token();
            }
            _ => {
                panic!("nodes must wrap in braces");
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
                panic!("nodes must wrap in braces");
            }
        }
        return nodes;
    }
    #[inline]
    fn parse_declar_or_rule(&mut self) -> Child<'source_str> {
        // name would be declaration prop or select, start with start_position.index, end with end_of_name.
        let mut start_index_of_name: Option<usize> = None;
        let mut end_index_of_name: usize = 0;
        let mut is_start_flag = false;
        let mut is_space_or_changeline_between = false;
        let mut is_space_or_newline = false;
        loop {
            match self.get_token() {
                Token::Colon  => {
                    // maybe selector start with colon (:root)
                    // maybe selector has colon (.class:hover)
                    // maybe is declaration ()
                    return self.parse_start_with_colon(start_index_of_name, is_space_or_newline, end_index_of_name);
                }
                Token::BracesLeft => {
                    match start_index_of_name {
                        Some(start_index_not_none) => {
                            let selector = self.lexer.get_sub_str(start_index_not_none, end_index_of_name);
                            return Child::Rule(self.parse_rule_with_selector( 
                                Cow::Borrowed(selector),
                                start_index_not_none
                            ));
                        }
                        None => {
                            let start_index_not_none = self.get_start_byte_index();
                            let selector = self.lexer.get_sub_str(start_index_not_none, start_index_not_none);
                            return Child::Rule(self.parse_rule_with_selector(
                                Cow::Borrowed(selector),
                                start_index_not_none,
                            ));
                        }
                    }
                }
                Token::NewLine | Token::Space => {
                    if is_start_flag {
                        is_space_or_changeline_between = true;
                    }
                    self.next_token();
                }
                _ => {
                    match start_index_of_name {
                        None => { start_index_of_name = Some(self.get_start_byte_index()) }
                        Some(_) => {}
                    }
                    if is_start_flag && is_space_or_changeline_between {
                        is_space_or_newline = true;
                    }
                    self.next_token();
                    end_index_of_name = self.get_start_byte_index();
                    is_start_flag = true;
                }
            }
        }
    }
    // start with colon maybe is
    // 1. selector start with colon: `:root`
    // 2. selector have colon: `.some-class:hover`
    // 3. declaration: `color: blue`
    fn parse_start_with_colon(&mut self, mut start_byte_index : Option<usize>, is_space_or_newline: bool, maybe_end_index_of_prop: usize) -> Child<'source_str> {
        let finish_byte_index: usize;
        match self.get_token() {
            Token::Colon => {
                match start_byte_index {
                    Some(_) => {},
                    None => {
                        start_byte_index = Some(self.get_start_byte_index());
                    }
                }
                self.next_token();
            }
            _ => {
                panic!("Declaration must have colon");
            }
        }
        let mut is_start_flag = false;
        let mut start_index_of_value: usize = 0; 
        let mut end_index_of_selector_or_value: usize = 0;
        loop {
            match self.get_token() {
                Token::BracesLeft | Token::BracesRight => {
                    let selector =self.lexer.get_sub_str(start_index_of_value ,end_index_of_selector_or_value);
                    return Child::Rule(self.parse_rule_with_selector( 
                        Cow::Borrowed(selector),
                        self.get_start_byte_index(),
                    ));
                }
                Token::Semi => {
                    break;
                }
                Token::NewLine | Token::Space => {
                    self.next_token();
                }
                _ => {
                    if !is_start_flag {
                        is_start_flag = true;
                        start_index_of_value = self.get_start_byte_index();
                    }
                    end_index_of_selector_or_value = self.get_finish_byte_index();
                    self.next_token();
                }
            }
        }
        match self.get_token() {
            Token::Semi => {
                finish_byte_index = self.get_finish_byte_index();
                self.next_token();
            }
            _ => {
                panic!("Declaration must end with semi");
            }
        }
        if is_space_or_newline {
            panic!("Declaration prop can not have space or new line");
        }
        return Child::Declar(Declaration { 
            prop: Cow::Borrowed(self.lexer.get_sub_str(start_byte_index.unwrap(),maybe_end_index_of_prop)),
            value: Cow::Borrowed(self.lexer.get_sub_str(start_index_of_value, end_index_of_selector_or_value)),
            start_byte_index: start_byte_index.unwrap(),
            finish_byte_index 
        });
    }
    #[inline]
    fn parse_rule_with_selector(&mut self, selector: Cow<'source_str, str>, start_byte_index: usize) -> Rule<'source_str> {
        match self.get_token() {
            Token::BracesLeft => {
                let nodes = self.parse_nodes_in_braces();
                Rule { 
                    selector,
                    nodes, 
                    start_byte_index,
                    finish_byte_index: self.get_finish_byte_index(),
                 }
            }
            _ => {
                panic!("rule must have brace " );
            }
        }
    }
}