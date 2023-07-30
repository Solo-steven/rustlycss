use std::borrow::Cow;
use serde_derive::{Deserialize, Serialize};
use crate::position::{Span, Location};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Declaration<'source_str> {
    pub prop: Cow<'source_str, str>,
    pub value: Cow<'source_str, str>,
    pub span: Span,
    pub loc: Location,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Rule<'a> {
    pub selector: Cow<'a, str>,
    #[serde(borrow)]
    pub nodes: Vec<Child<'a>>,
    pub span: Span,
    pub loc: Location,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AtRule<'a> {
    pub name: Cow<'a, str>,
    pub param: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub nodes: Option<Vec<Child<'a>>>,
    pub span: Span,
    pub loc: Location,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Child<'a> {
    AtRule(AtRule<'a>),
    #[serde(borrow)]
    Rule(Rule<'a>),
    Declar(Declaration<'a>)
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Root<'a> {
    #[serde(borrow)]
    pub nodes: Vec<Child<'a>>,
    pub span: Span,
    pub loc: Location,
}
