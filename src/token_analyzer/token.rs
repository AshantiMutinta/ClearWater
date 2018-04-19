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
    Numeric,
    Alphabetic,
    Space,
    SpecialCharacter,
    EndingTerminator,
    AssignmentSymbol,
    Arithmentic,
    SinlgeQuote,
    Newline,
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

static ALPHABETIC: &'static str = r"[A-z]+$";
static NUMERIC: &'static str = r"[0-9]+$";
static ENDING_TERMINATOR: &'static str = ";";
static ASSIGNMENT_SYMBOL: &'static str = "=";
static ARITHMETIC_SYMBOL: &'static str = r"\+|-|/|<|>|\*";
static SINGLE_QUOTE: &'static str = "'";
static SPACE: &'static str = " ";
static NEWLINE: &'static str = "\n";

lazy_static! {
    pub static ref RULES: Vec<TokenRules<'static>> = {
        vec![
            TokenRules{
                token_type : &TokenType::Alphabetic,
                regex_rule : String::from(ALPHABETIC)
            },
            TokenRules{
                token_type : &TokenType::EndingTerminator,
                regex_rule : String::from(ENDING_TERMINATOR)
            },
            TokenRules{
                token_type : &TokenType::AssignmentSymbol,
                regex_rule : String::from(ASSIGNMENT_SYMBOL)
            },
            TokenRules{
                token_type : &TokenType::Arithmentic,
                regex_rule : String::from(ARITHMETIC_SYMBOL)
            },
            TokenRules{
                token_type : &TokenType::SinlgeQuote,
                regex_rule : String::from(SINGLE_QUOTE)
            },
            TokenRules{
                token_type : &TokenType::Space,
                regex_rule : String::from(SPACE)
            },
            TokenRules{
                token_type :&TokenType::Numeric,
                regex_rule : String::from(NUMERIC)
            },
            TokenRules{
                token_type : &TokenType::Newline,
                regex_rule : String::from(NEWLINE)
            }
        ]

    };

}
