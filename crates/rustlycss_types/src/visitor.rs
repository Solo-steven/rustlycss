use crate::ast::*;
pub trait Visitor {
    fn visit_mut_root(&mut self, _root: &mut Root) {}
    fn visit_mut_declaration(&mut self, _declaration:&mut Declaration) {}
    fn visit_mut_rule(&mut self, _rule: &mut Rule) {}
    fn visit_mut_at_rule(&mut self, _at_rule: &mut AtRule) {}
}

pub trait Walkable {
    fn visit_mut_children<V: Visitor>(&mut self, _visitor: &mut V) {}
}
impl<'a> Walkable for Declaration<'a> {}

impl<'a> Walkable for Rule<'a> {
    fn visit_mut_children<V: Visitor>(&mut self, visitor: &mut V) {
        for node in &mut self.nodes {
            match node {
                Child::AtRule(at_rule) => visitor.visit_mut_at_rule(at_rule),
                Child::Declar(declar) => visitor.visit_mut_declaration(declar),
                Child::Rule(rule) => visitor.visit_mut_rule(rule)
            }
        }
    }
}
impl<'a >Walkable for AtRule<'a> {
    fn visit_mut_children<V: Visitor>(&mut self, visitor: &mut V) {
        match self.nodes.as_mut() {
            Some(nodes) => {
                for node in nodes {
                    match node {
                        Child::AtRule(at_rule) => visitor.visit_mut_at_rule(at_rule),
                        Child::Declar(declar) => visitor.visit_mut_declaration(declar),
                        Child::Rule(rule) => visitor.visit_mut_rule(rule)
                    }
                }
            }
            None => {}
        }
    }
}
impl <'a> Walkable for Root<'a> {
    fn visit_mut_children<V: Visitor>(&mut self, visitor: &mut V) {
        for node in &mut self.nodes {
            match node {
                Child::AtRule(at_rule) => visitor.visit_mut_at_rule(at_rule),
                Child::Declar(declar) => visitor.visit_mut_declaration(declar),
                Child::Rule(rule) => visitor.visit_mut_rule(rule)
            }
        }
    }
}
