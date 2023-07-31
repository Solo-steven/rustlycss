use regex::Regex;
use std::{borrow::*, vec};
use rustlycss_types::position::{Location, Span};
use rustlycss_types::ast::*;

pub struct  NestedVisitor<'a> {
    new_rules: Vec<Child<'a>>,
    test: Regex
}
#[derive(Debug)]
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

impl<'a> NestedVisitor<'a> {
    pub fn new() -> Self {
        NestedVisitor { 
            new_rules: Vec::new(),
            test: Regex::new(r"&").unwrap()
        }
    }
    pub fn visit(&mut self, root: &mut Root<'a>) {
        self.accept_root(root);
    }
    fn accept_root<'b>(&mut self, root: &mut Root<'a>) {
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
        match &mut root.nodes {
            Some(nodes) => {
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
            None => {}
        }
        // if atrule is media.
        // 1. put all new_node into media's nodes vec
        // 2. if there are any declaration, create a new rule node to wrap declars.
        match root.name.as_ref() {
            "media" => {
                match root.nodes.as_mut() {
                    Some(nodes) => {
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
                        if declars.len() != 0 {
                            for delcar in declars {
                                wrapper_node.nodes.push(Child::Declar(delcar));
                            }
                            nodes.push(Child::Rule(wrapper_node));
                        }
                    },
                    None => {}
                }
            }
            _ => {
                match &mut root.nodes {
                    Some(nodes) => {
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
                                    return false;
                                }
                            }
                        })
                    }
                    None => {}
                }
            }
        };
        return Action::ToTopLevel;
    }
    fn accept_rule(&mut self, root: &mut Rule<'a>, prefix:&str) -> Action {
        // if selector contain &, replace it with prefix rule name
        // if is not contain &, prefix should add current selector to the s
        if root.selector.contains('&') {
            let new_selector_string = String::from(self.test.replace_all(root.selector.as_ref(), prefix));
            root.selector = Cow::Owned(new_selector_string);
        } else {
            if prefix.len() != 0 {
                let mut new_selector_string = String::from(prefix);
                new_selector_string.push_str(" ");
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
                    return false;
                }
            }
        });
        return  if count == 0 { Action::Remove } else if prefix.len() != 0 { Action::ToTopLevel }  else  { Action::Keep }
    }
}

