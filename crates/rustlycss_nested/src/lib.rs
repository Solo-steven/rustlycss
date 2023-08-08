#[allow(clippy::all)]
use regex::Regex;
use std::{borrow::*, vec};
use rustlycss_types::position::{Location, Span};
use rustlycss_types::ast::*;

pub struct  NestedVisitor<'a> {
    new_rules: Vec<Child<'a>>,
    ampersand_regex: Regex,
    string_literal_regex: Regex
}
#[derive(Debug, PartialEq)]
enum Action {
    Remove,
    Keep,
    ToTopLevel,
}

#[inline]
fn create_dummy_declaration() -> Declaration<'static> {
    Declaration { 
        prop: Cow::Borrowed(""),
        value: Cow::Borrowed(""),
        span: Span::new(), 
        loc: Location::new() 
    }
}
#[inline]
fn create_dummy_rule_with_str(prefix: &str) -> Rule<'static> {
    Rule { 
        selector: Cow::Owned(String::from(prefix)),
        nodes: vec![], 
        span: Span::new(), 
        loc: Location::new() 
    }
}

impl<'a> Default for NestedVisitor<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> NestedVisitor<'a> {
    pub fn new() -> Self {
        NestedVisitor { 
            new_rules: Vec::new(),
            ampersand_regex: Regex::new(r"&").unwrap(),
            string_literal_regex: Regex::new("[\"\'][^\'\"]*&[^\"\']*[\"\']").unwrap(),
        }
    }
    pub fn visit(&mut self, root: &mut Root<'a>) {
        self.accept_root(root);
    }
    fn accept_root(&mut self, root: &mut Root<'a>) {
        // iterate over children, apply action according to return
        let mut children_actions: Vec<Action> = Vec::with_capacity(root.nodes.len());
        for node in &mut root.nodes {
            match node {
                Child::Declar(_declar) => {
                    children_actions.push(Action::Remove);
                }
                Child::Rule(rule) => {
                    children_actions.push(self.accept_rule(rule, ""));
                }
                Child::AtRule(at_rule) => {
                    children_actions.push(self.accept_at_rule(at_rule, ""));
                }
            }
        };
        let mut index: usize = 0;
        root.nodes.retain(|_node| {
            index+= 1;
            match children_actions[index-1] {
                Action::Remove => false,
                Action::Keep => true,
                Action::ToTopLevel => true
            }
        });
        root.nodes.append(&mut self.new_rules);
    }
    fn accept_at_rule(&mut self, root: &mut AtRule<'a>, prefix: &str) -> Action {
        // iterate over children, apply action according to return
        let mut children_actions: Vec<Action> = Vec::new();
        if let Some(nodes) = root.nodes.as_mut() {
            for node in nodes {
                match node {
                    Child::Declar(_declar) => {
                        children_actions.push(Action::Keep);
                    }
                    Child::Rule(rule) => {
                        children_actions.push(self.accept_rule(rule, prefix));
                    }
                    Child::AtRule(at_rule) => {
                        children_actions.push(self.accept_at_rule(at_rule, prefix));
                    }
                }
            }
        }
        // if atrule is media.
        // 1. put all new_node into media's nodes vec
        // 2. if there are any declaration, create a new rule node to wrap declars.
        match root.name.as_ref() {
            "media" | "supports"  => {
                if let Some(nodes) = root.nodes.as_mut() {
                    let mut declars = Vec::<Declaration<'a>>::new();
                    let mut index = 0;
                    nodes.retain_mut(|node| {
                        index += 1;
                        match children_actions[index-1] {
                            Action::Remove => false,
                            Action::ToTopLevel => true,
                            Action::Keep => {
                                match node {
                                    Child::Declar(declar) => {
                                        declars.push(
                                            std::mem::replace(
                                                declar, 
                                                create_dummy_declaration()
                                            )
                                        );
                                        false
                                    }
                                    _ => true
                                }
                            },
                        }
                    });
                    nodes.append(&mut self.new_rules);
                   // self.new_rules = Vec::new();
                    let mut wrapper_node = create_dummy_rule_with_str(prefix);
                    if !declars.is_empty() {
                        for delcar in declars {
                            wrapper_node.nodes.push(Child::Declar(delcar));
                        }
                        nodes.push(Child::Rule(wrapper_node));
                    }
                    return Action::ToTopLevel;
                }
            }
            _ => {
                if let Some(nodes) = root.nodes.as_mut() {
                    let mut index: usize = 0;
                    nodes.retain_mut(|node| {
                        index += 1;
                        match children_actions[index-1] {
                            Action::Remove => false,
                            Action::Keep => true,
                            Action::ToTopLevel => {
                                self.new_rules.push(
                                    std::mem::replace(
                                        node, 
                                        Child::Declar(create_dummy_declaration())
                                    )
                                );
                                false
                            }
                        }
                    });
                    return match root.name.as_ref() {
                        "font-face" | "keyframes" => Action::ToTopLevel,
                        _ => Action::Keep
                    }
                }
            }
        };
        Action::Keep
    }
    fn accept_rule(&mut self, root: &mut Rule<'a>, prefix:&str) -> Action {
        let is_selectors = root.selector.split(',').count() > 1; 
        if is_selectors {
            let selectors = root.selector.split(',');
            for selector_not_trim in selectors {
                let selector = selector_not_trim.trim();
                let mut new_root = root.clone();
                new_root.selector = Cow::Owned(String::from(selector));
                let action = self.accept_rule(&mut new_root, prefix);
                if Action::ToTopLevel == action {
                    self.new_rules.push(Child::Rule(new_root))
                }else {
                    //logical error
                }
            }
            return Action::Remove;
        }else {
            // if selector contain &, replace it with prefix rule name
            // if is not contain &, prefix should add current selector to the s
            if root.selector.contains('&') {
                // if there are any string literal contain &, replace it as `__rustly_css_ts_{index}` frist
                let mut new_selector_string = String::from(root.selector.as_ref());
                let match_string_literals: Vec<_> = self.string_literal_regex.find_iter(&root.selector).map(|m| m.as_str()).collect();
                for (index, match_string_literal) in match_string_literals.iter().enumerate() {
                    let current_test = Regex::new(match_string_literal).unwrap();
                    new_selector_string = String::from(current_test.replace_all(&new_selector_string, format!("'__rustlycss_ts_{}'", index)).as_ref());
                }
                new_selector_string = String::from(self.ampersand_regex.replace_all(new_selector_string.as_ref(), prefix));
                // if there are any string literal be replaced `__rustly_css_ts_{index}`, replace back to original string
                for (index, match_string_literal) in match_string_literals.iter().enumerate() {
                    let current_test = Regex::new(format!("'__rustlycss_ts_{}'", index).as_str()).unwrap();
                    new_selector_string = String::from(current_test.replace_all(&new_selector_string, *match_string_literal).as_ref());
                }
                root.selector = Cow::Owned(new_selector_string);
            } else if !prefix.is_empty() {
                let mut new_selector_string = String::from(prefix);
                new_selector_string.push(' ');
                new_selector_string.push_str(root.selector.as_ref());
                root.selector = Cow::Owned(new_selector_string);
            }
        }
        // iterate over children, apply action according to return
        let mut children_actions: Vec<Action> = Vec::with_capacity(root.nodes.len());
        for node in &mut root.nodes {
            match node {
                Child::Declar(_declar) => {
                    children_actions.push(Action::Keep);
                }
                Child::Rule(rule) => {
                    children_actions.push(self.accept_rule(rule, &root.selector));
                }
                Child::AtRule(at_rule) => {
                    children_actions.push(self.accept_at_rule(at_rule, &root.selector));
                }
            }
        }
        let mut index: usize = 0;
        let mut count: usize = 0;
        root.nodes.retain_mut(|node| {
            index+=1;
            match children_actions[index-1] {
                Action::Remove => false,
                Action::Keep => { 
                    count += 1;
                    true
                },
                Action::ToTopLevel => {
                    self.new_rules.push(
                        std::mem::replace(
                            node, 
                            Child::Declar(create_dummy_declaration())
                        )
                    );
                    false
                }
            }
        });
        if count == 0 { Action::Remove } else if !prefix.is_empty() { Action::ToTopLevel }  else  { Action::Keep }
    }
}

