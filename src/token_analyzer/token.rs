use std::cmp::Ord;
use std::cmp::Ordering;

#[derive(Eq)]
pub struct TokenMatch<'a> {
    pub literal: String,
    pub begin_segmet: usize,
    pub end_segment: usize,
    pub rule: &'a TokenRules<'a>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum TokenError {
    EmptyLineOfCode,
    MatchingError,
    NoMatch,
}

#[derive(PartialEq, Eq, Debug)]
pub enum TokenType {
    Alphanumeric,
    Literal,
    Number,
    Keyword,
    Space,
    SpecialCharacter,
    EndingTerminator,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Token<'a> {
    pub content: String,
    pub token_type: &'a TokenType,
}

#[derive(PartialEq, Eq)]
pub struct TokenRules<'a> {
    pub token_type: &'a TokenType,
    pub regex_rule: String,
}

impl<'a> Ord for TokenMatch<'a> {
    fn cmp(&self, other: &TokenMatch) -> Ordering {
        self.begin_segmet.cmp(&other.begin_segmet)
    }
}

impl<'a> PartialOrd for TokenMatch<'a> {
    fn partial_cmp(&self, other: &TokenMatch) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for TokenMatch<'a> {
    fn eq(&self, other: &TokenMatch) -> bool {
        (self.literal == other.literal) && (self.begin_segmet == other.begin_segmet)
            && (self.end_segment == other.end_segment)
    }
}

pub static KEYWORD: &'static str = r"[A-z]+$";

pub static ENDING_TERMINATOR: &'static str = ";";

lazy_static! {
    pub static ref RULES: Vec<TokenRules<'static>> = {
        vec![
            TokenRules{
                token_type : &TokenType::Keyword,
                regex_rule : String::from(KEYWORD)
            },
            TokenRules{
                token_type : &TokenType::EndingTerminator,
                regex_rule : String::from(ENDING_TERMINATOR)
            }
        ]

    };

}
