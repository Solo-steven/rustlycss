pub mod source_map;

use rustlycss_types::ast::*;
use rustlycss_types::config::GeneralConfig;
use rustlycss_types::position::Position;

pub struct Generator<'a> {
    pub output: String,
    pub mapping: Vec<(Position, Position)>,
    pos: Position,
    depth: usize,
    config: &'a GeneralConfig
}

impl<'a> Generator<'a> {
    pub fn new(config: &'a GeneralConfig) -> Self {
        Generator {
            output: String::with_capacity(1200),
            pos: Position::new(),
            mapping: Vec::with_capacity(500),
            depth: 0,
            config,
        }
    }
    pub fn generate(&mut self, ast: &Root) {
        self.accept_root(ast);
    }
    // util function for write a value into string
    #[inline]
    fn write_str(&mut self,value: &str) {
        if self.config.sourcemap {
            self.pos.col += value.chars().count();
        }
        self.output.push_str(value);
    }
    #[inline]
    fn write_changeline(&mut self) {
        if self.config.minify {
            return;
        }
        self.pos.row += 1;
        self.pos.col = 0;
        self.write_str("\n");
    }
    #[inline]
    fn write_space(&mut self) {
        if self.config.minify {
            return ;
        }
        for _i in 0..self.depth {
            self.write_str("   ");
        }
    }
    #[inline]
    fn add_mapping(&mut self, orig_pos: Position, gen_pos: Position) {
        if !self.config.sourcemap {
            return;
        }
        self.mapping.push((orig_pos, gen_pos));
    }
    #[inline]
    fn accept_child(&mut self, child: &Child) {
        match *child {
            Child::AtRule(ref at_rule_node) => self.accept_at_rule(at_rule_node),
            Child::Rule(ref rule_node) => self.accept_rule(rule_node),
            Child::Declar(ref declar_node) => self.accept_declaration(declar_node)
        }
    }
    #[inline]
    fn accept_root(&mut self, root: &Root) {
        self.add_mapping(root.loc.start.clone(), self.pos.clone());
        for node in &root.nodes {
            self.accept_child(node)
        }
        self.add_mapping(root.loc.finish.clone(), self.pos.clone());
    }
    #[inline]
    fn accept_declaration(&mut self, root: &Declaration) {
        self.write_space();
        self.add_mapping(root.loc.start.clone(), self.pos.clone());
        self.write_str(root.prop.as_ref());
        self.write_str(":");
        self.write_str(" ");
        self.write_str(root.value.as_ref());
        self.write_str(";");
        self.add_mapping(root.loc.finish.clone(), self.pos.clone());
        self.write_changeline();
    }
    fn accept_rule(&mut self, root: &Rule) {
        self.write_space();
        self.add_mapping(root.loc.start.clone(), self.pos.clone());
        self.write_str(root.selector.as_ref());
        self.write_str(" ");
        self.write_str("{");
        self.write_changeline();
        self.depth += 1;
        for node in &root.nodes {
            self.accept_child(node)
        }
        self.depth -= 1;
        self.write_space();
        self.write_str("}");
        self.write_changeline();
        self.add_mapping(root.loc.finish.clone(), self.pos.clone());
    }
    fn accept_at_rule(&mut self, root: &AtRule) {
        self.write_space();
        self.add_mapping(root.loc.start.clone(), self.pos.clone());
        self.write_str("@");
        self.write_str(root.name.as_ref());
        match &root.param {
            Some(param) => {
                self.write_str(" ");
                self.write_str(param.as_ref());
            }
            None => {}
        }
        match &root.nodes {
            Some(nodes) => {
               self.write_str(" ");
               self.write_str("{");
               self.write_changeline();
               self.depth += 1;
                for node in nodes {
                    self.accept_child(node)
                }
                self.depth -= 1;
                self.write_space();
                self.write_str("}");
                self.write_changeline();
            }
            None => {}
        }
        self.add_mapping(root.loc.finish.clone(), self.pos.clone());
        return;
    }
}