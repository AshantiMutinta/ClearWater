extern crate regex;

use self::regex::Regex;
use std::collections::BinaryHeap;
use token_analysis::token;

fn tokenize_source_code(
    source_code: String,
) -> Vec<Result<Vec<token::Token<'static>>, token::TokenError>> {
    source_code
        .split(";")
        .map(|line_of_code| tokenize_line(String::from(line_of_code), &token::RULES))
        .collect::<Vec<_>>()
}

fn addTokensToBinaryHeap<'a>(
    line_of_code: &String,
    match_heap: &mut BinaryHeap<token::TokenMatch<'a>>,
    token_rules: &'a Vec<token::TokenRules>,
) {
    token_rules
        .iter()
        .map(|token_match_rule| {
            Regex::new(&token_match_rule.regex_rule)
                .map(|regex| {
                    let matched_rules = regex.find_iter(&line_of_code);
                    matched_rules
                        .map(|t| {
                            match_heap.push(token::TokenMatch {
                                literal: String::from(t.as_str()),
                                begin_segmet: t.start().clone(),
                                end_segment: t.end().clone(),
                                rule: token_match_rule,
                            });
                        })
                        .collect::<Vec<_>>();
                })
                .map_err(|_| ());
        })
        .collect::<Vec<_>>();
}

fn slice_line_of_code<'a>(first_match: token::TokenMatch, line_of_code: &String) -> String {
    let mut mutable_line_of_code = line_of_code.clone();
    mutable_line_of_code.split_off(first_match.begin_segmet);
    let end_range = line_of_code.clone().split_off(first_match.end_segment);
    mutable_line_of_code.push_str(&end_range);
    mutable_line_of_code
}
fn tokenize_line<'a>(
    line_of_code: String,
    token_rules: &'a Vec<token::TokenRules>,
) -> Result<Vec<token::Token<'a>>, token::TokenError> {
    if line_of_code.len() <= 0 {
        Err(token::TokenError::EmptyLineOfCode)
    } else {
        let mut tokens = vec![];
        let mut match_heap = BinaryHeap::new();
        addTokensToBinaryHeap(&line_of_code, &mut match_heap, token_rules);
        match_heap
            .pop()
            .ok_or_else(|| token::TokenError::NoMatch)
            .and_then(|first_match| {
                tokens.push(token::Token {
                    content: first_match.literal.clone(),
                    token_type: &first_match.rule.token_type,
                });
                tokenize_line(slice_line_of_code(first_match, &line_of_code), token_rules)
                    .map(|tokenized| {
                        tokens.extend(tokenized);
                        Ok(tokens.clone())
                    })
                    .unwrap_or(Ok(tokens.clone()))
            })
    }
}

#[test]
fn test_ending_terminator() {
    let tokens = tokenize_line(String::from(";;"), &token::RULES).expect("expect tokens");
    assert_eq!(tokens.len(), 2);

    let first_token = tokens.first().expect("expected token after first");
    assert_eq!(first_token.content, ";");
    assert_eq!(first_token.token_type, &token::TokenType::EndingTerminator);
}

#[test]
fn test_alphabetic() {
    let tokens =
        tokenize_line(String::from("thisforme;again"), &token::RULES).expect("expect tokens");
    assert_eq!(tokens.len(), 3);

    let first_token = tokens.first().expect("expected token after first");
    assert_eq!(first_token.content, "again");
    assert_eq!(first_token.token_type, &token::TokenType::Alphabetic);

    let last_token = tokens.get(2).expect("expected token after get");
    assert_eq!(last_token.content, "thisforme");
    assert_eq!(last_token.token_type, &token::TokenType::Alphabetic);
}

#[test]
fn test_assignment_symbol() {
    let tokens = tokenize_line(String::from("thisforme;="), &token::RULES).expect("expect tokens");
    assert_eq!(tokens.len(), 3);

    let last_token = tokens.get(0).expect("expected token after get");
    assert_eq!(last_token.content, "=");
    assert_eq!(last_token.token_type, &token::TokenType::AssignmentSymbol);
}

#[test]
fn test_numeric_symbol() {
    let tokens =
        tokenize_line(String::from("thisforme;=1234"), &token::RULES).expect("expect tokens");
    assert_eq!(tokens.len(), 4);

    let last_token = tokens.get(0).expect("expected token after get");
    assert_eq!(last_token.content, "1234");
    assert_eq!(last_token.token_type, &token::TokenType::Numeric);
}

#[test]
fn test_newline_symbol() {
    let tokens =
        tokenize_line(String::from("thisforme;=1234\n"), &token::RULES).expect("expect tokens");
    assert_eq!(tokens.len(), 5);

    let last_token = tokens.get(0).expect("expected token after get");
    assert_eq!(last_token.content, "\n");
    assert_eq!(last_token.token_type, &token::TokenType::Newline);
}

#[test]
fn test_space() {
    let tokens = tokenize_line(String::from("  try"), &token::RULES).expect("expect tokens");
    assert_eq!(tokens.len(), 3);

    let last_token = tokens.get(2).expect("expected token after get");
    assert_eq!(last_token.content, " ");
    assert_eq!(last_token.token_type, &token::TokenType::Space);

    let middle_token = tokens.get(1).expect("expected token after get");
    assert_eq!(middle_token.content, " ");
    assert_eq!(middle_token.token_type, &token::TokenType::Space);

    let first_token = tokens.get(0).expect("expected token after get");
    assert_eq!(first_token.content, "try");
    assert_eq!(first_token.token_type, &token::TokenType::Alphabetic);
}

#[test]
fn test_single_quote() {
    let tokens = tokenize_line(String::from("' tryme'"), &token::RULES).expect("expect tokens");
    assert_eq!(tokens.len(), 4);

    let last_token = tokens.get(3).expect("expected token after get");
    assert_eq!(last_token.content, "'");
    assert_eq!(last_token.token_type, &token::TokenType::SinlgeQuote);

    let middle_token = tokens.get(2).expect("expected token after get");
    assert_eq!(middle_token.content, " ");
    assert_eq!(middle_token.token_type, &token::TokenType::Space);

    let first_token = tokens.get(0).expect("expected token after get");
    assert_eq!(first_token.content, "'");
    assert_eq!(first_token.token_type, &token::TokenType::SinlgeQuote);
}

#[test]
fn test_artihmetic_symbol() {
    let tokens = tokenize_line(String::from("stuff<try>*okay-;+this/j:"), &token::RULES)
        .expect("expect tokens");
    assert_eq!(tokens.len(), 11);
    let mut test = vec!["/", "+", "-", "*", ">", "<", ":"];
    test.reverse();
    let filtered_tokens = tokens
        .iter()
        .filter(|filt| test.contains(&&*filt.content))
        .collect::<Vec<_>>();
    assert_eq!(filtered_tokens.len(), test.len());
    filtered_tokens
        .iter()
        .map(|t| {
            assert_eq!(t.content, test.pop().expect("error unwrapping test data"));
            assert_eq!(t.token_type, &token::TokenType::Arithmentic);
        })
        .collect::<Vec<_>>();
}
