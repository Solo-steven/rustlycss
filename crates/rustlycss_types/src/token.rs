#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Start,          // token for null
    At,             // @
    Semi,           // ;
    Space,          // space, \t
    NewLine,        // \n
    Colon,          // :
    Comma,          // ,
    BracesLeft,     // {
    BracesRight,    // }
    BracketLeft,    // [
    BracketRight,   // ]
    ParenthesesLeft,    // (
    ParenthesesRight,   // )
    Word,
    Comment,
    StringLiteral,
    EOF,            // eof
}

pub const AT_CHAR: char = '@';
pub const SEMI_CHAR: char = ';';
pub const TAB_CHAR: char = '\t';
pub const NEWLINE_CHAR: char = '\n';
pub const SPACE_CHAR: char = ' ';
pub const COLON_CHAR: char = ':';
pub const COMMA_CHAR: char = ',';
pub const BRACES_LEFT_CHAR: char = '{';
pub const BRACES_RIGHT_CHAR: char = '}';
pub const BRACKET_LEFT_CHAR: char = '[';
pub const BRACKET_RIGHT_CHAR: char = ']';
pub const PARENTHESES_LEFT_CHAR: char = '(';
pub const PARENTHESES_RIGHT_CHAR: char = ')';
pub const SINGLE_QUOTE: char = '\'';
pub const DOUBLE_QUOTE: char = '\"';

