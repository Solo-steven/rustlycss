use rustlycss_types::ast::*;
use std::borrow::Cow;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct SimpleVarVisitor<'a> {
    dollar_sign_regex: Regex,
    dollar_sign_with_parentheses_regex: Regex,
    cache: HashMap<Cow<'a, str>, Cow<'a, str>>,
    
}
impl <'a> Default for SimpleVarVisitor<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl<'a>  SimpleVarVisitor<'a> {
    pub fn new() -> Self {
        Self {
            dollar_sign_regex: Regex::new(r"\$.*").unwrap(),
            dollar_sign_with_parentheses_regex: Regex::new(r"\$\([^\s]*\)").unwrap(),
            cache: HashMap::new(),
        }
    }
    pub fn visit(&mut self, root: &mut Root<'a>) {
        self.accept_root(root);
    }   
    fn accept_root(&mut self, root: &mut Root<'a>) {
        for node in &mut root.nodes {
            match node {
                Child::Declar(declar) => self.accept_declaration(declar),
                Child::AtRule(at_rule) => self.accept_at_rule(at_rule),
                Child::Rule(rule) => self.accept_rule(rule),
            }
        }
    }
    fn accept_at_rule(&mut self, root: &mut AtRule<'a>) {
        if let Some(nodes) = root.nodes.as_mut() {
            for node in nodes {
                match node {
                    Child::Declar(declar) => self.accept_declaration(declar),
                    Child::AtRule(at_rule) => self.accept_at_rule(at_rule),
                    Child::Rule(rule) => self.accept_rule(rule),
                }
            }
        }
    } 
    fn accept_rule(&mut self, root: &mut Rule<'a>) {
        let mut new_selector = String::from(root.selector.as_ref());
        if root.selector.starts_with('$') {
            if let Some(replace_value) = self.cache.get(root.selector.as_ref()) {
                let current_test = Regex::new(regex::escape(root.selector.as_ref()).as_str()).unwrap();
                new_selector = String::from(current_test.replace_all(&new_selector, replace_value));

            }
            root.selector = Cow::Owned(new_selector);
        } else if self.dollar_sign_with_parentheses_regex.is_match(root.selector.as_ref()) {
            let all_match_dollar_sign_with_parentheses: Vec<_> = self.dollar_sign_with_parentheses_regex.find_iter(root.selector.as_ref()).map(|m| m.as_str()).collect();
            for match_dollar_sign_with_parentheses in all_match_dollar_sign_with_parentheses {
                if let Some(replace_value) = self.cache.get(match_dollar_sign_with_parentheses) {
                    let current_test =  Regex::new(regex::escape(match_dollar_sign_with_parentheses).as_str()).unwrap();
                    new_selector = String::from(current_test.replace_all(&new_selector, replace_value.as_ref()))
                }
            }
            root.selector = Cow::Owned(new_selector);
        }
        for node in &mut root.nodes {
            match node {
                Child::Declar(declar) => self.accept_declaration(declar),
                Child::AtRule(at_rule) => self.accept_at_rule(at_rule),
                Child::Rule(rule) => self.accept_rule(rule),
            }
        }
    }
    fn accept_declaration(&mut self, root: &mut Declaration<'a>) {
        if root.prop.starts_with('$') {
            let dollar_sign_off_str = &root.prop[1..];
            let dollar_sign_str_with_parentheses = Cow::Owned(format!("$({})", dollar_sign_off_str));
            self.cache.insert(root.prop.clone(), root.value.clone());
            self.cache.insert(dollar_sign_str_with_parentheses, root.value.clone());
        }
        let is_value_match_dollar_sign = self.dollar_sign_regex.is_match(root.value.as_ref());
        let is_value_match_dollar_sign_with_parentheses = self.dollar_sign_with_parentheses_regex.is_match(root.value.as_ref());
        if is_value_match_dollar_sign || is_value_match_dollar_sign_with_parentheses {
            let mut new_value = String::from(root.value.as_ref());
            let all_match_dollar_sign: Vec<_> = self.dollar_sign_regex.find_iter(root.value.as_ref()).map(|m| m.as_str()).collect();
            let all_match_dollar_sign_with_parentheses: Vec<_> = self.dollar_sign_with_parentheses_regex.find_iter(root.value.as_ref()).map(|m| m.as_str()).collect();
            for match_dollar_sign in all_match_dollar_sign {
                if let Some(replace_value) = self.cache.get(match_dollar_sign) {
                    let current_test =  Regex::new(regex::escape(match_dollar_sign).as_str()).unwrap();
                    new_value = String::from(current_test.replace_all(&new_value, replace_value.as_ref()))
                }
            }
            for match_dollar_sign_with_parentheses in all_match_dollar_sign_with_parentheses {
                if let Some(replace_value) = self.cache.get(match_dollar_sign_with_parentheses) {
                    let current_test =  Regex::new(regex::escape(match_dollar_sign_with_parentheses).as_str()).unwrap();
                    new_value = String::from(current_test.replace_all(&new_value, replace_value.as_ref()))
                }
            }
            root.value = Cow::Owned(new_value); 
        }
        let is_match_dollar_sign = self.dollar_sign_regex.is_match(root.prop.as_ref());
        let is_match_dollar_sign_with_parentheses = self.dollar_sign_with_parentheses_regex.is_match(root.prop.as_ref());
        if is_match_dollar_sign || is_match_dollar_sign_with_parentheses {
            let mut new_value = String::from(root.value.as_ref());
            let all_match_dollar_sign: Vec<_> = self.dollar_sign_regex.find_iter(root.prop.as_ref()).map(|m| m.as_str()).collect();
            let all_match_dollar_sign_with_parentheses: Vec<_> = self.dollar_sign_with_parentheses_regex.find_iter(root.prop.as_ref()).map(|m| m.as_str()).collect();
            for match_dollar_sign in all_match_dollar_sign {
                if let Some(replace_value) = self.cache.get(match_dollar_sign) {
                    let current_test =  Regex::new(regex::escape(match_dollar_sign).as_str()).unwrap();
                    new_value = String::from(current_test.replace_all(&new_value, replace_value.as_ref()))
                }
            }
            for match_dollar_sign_with_parentheses in all_match_dollar_sign_with_parentheses {
                if let Some(replace_value) = self.cache.get(match_dollar_sign_with_parentheses) {
                    let current_test =  Regex::new(regex::escape(match_dollar_sign_with_parentheses).as_str()).unwrap();
                    new_value = String::from(current_test.replace_all(&new_value, replace_value.as_ref()))
                }
            }
            root.prop = Cow::Owned(new_value); 
        }
    }
}

