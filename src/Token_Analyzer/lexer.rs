extern crate regex;

use self::regex::Regex;
use std::collections::BinaryHeap;
use token_analyzer::token;

fn tokenize_line<'a>(
    line_of_code: String,
    token_rules: &'a Vec<token::TokenRules>,
) -> Result<Vec<token::Token<'a>>, token::TokenError> {
    if line_of_code.len() <= 0 {
        Err(token::TokenError::EmptyLineOfCode)
    } else {
        let mut tokens = vec![];
        let mut match_heap = BinaryHeap::new();
        token_rules
            .iter()
            .map(|token_match_rule| {
                match Regex::new(&token_match_rule.regex_rule) {
                    Ok(regex) => {
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
                    }
                    Err(_) => {
                        ();
                    }
                };
            })
            .collect::<Vec<_>>();
        match match_heap.pop() {
            Some(first_match) => {
                tokens.push(token::Token {
                    content: first_match.literal.clone(),
                    token_type: &first_match.rule.token_type,
                });

                let mut mutable_line_of_code = line_of_code.clone();
                mutable_line_of_code.split_off(first_match.begin_segmet);
                let mut end_range = line_of_code.clone().split_off(first_match.end_segment);
                mutable_line_of_code.push_str(&end_range);
                match tokenize_line(mutable_line_of_code, token_rules) {
                    Ok(tokenized) => {
                        tokens.extend(tokenized);
                        Ok(tokens)
                    }
                    Err(_) => Ok(tokens),
                }
            }
            None => Err(token::TokenError::NoMatch),
        }
    }
}

#[test]
fn test_ending_terminator() {
   

    let tokens = tokenize_line(String::from(" ; ; "), &token::RULES).expect("expect tokens");
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
    assert_eq!(first_token.token_type, &token::TokenType::Keyword);

    let last_token = tokens.get(2).expect("expected token after get");
    assert_eq!(last_token.content, "thisforme");
    assert_eq!(last_token.token_type, &token::TokenType::Keyword);
}
