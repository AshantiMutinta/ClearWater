extern crate regex;

use self::regex::Regex;
use std::collections::BinaryHeap;
use std::cmp::Ord;
use std::cmp::Ordering;




#[derive(PartialEq,Eq,Debug)]
enum TokenError
{
    empty_line_of_code,
    matching_error,
    no_match
}

#[derive(PartialEq,Eq,Debug)]
enum TokenType
{
    alphanumeric,
    literal,
    number,
    keyword,
    space,
    special_character,
    ending_terminator
}

#[derive(PartialEq,Eq,Debug)]
struct Token<'a>
{
    content : String,
    token_type : &'a TokenType
}

#[derive(PartialEq,Eq)]
struct TokenRules<'a>
{
    token_type : &'a TokenType,
    regex_rule: String,
}

#[derive(Eq)]
struct TokenMatch<'a>
{
    literal : String,
    begin_segmet : usize,
    end_segment : usize,
    rule : &'a TokenRules<'a>
}

impl<'a> Ord for TokenMatch<'a>
{
    fn cmp(&self,other:&TokenMatch) -> Ordering
    {
        self.begin_segmet.cmp(&other.begin_segmet)
    }
}


impl<'a> PartialOrd for TokenMatch<'a> {
    fn partial_cmp(&self, other: &TokenMatch) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for TokenMatch<'a> {
    fn eq(&self, other: &TokenMatch) -> bool 
    {
        (self.literal == other.literal) && 
        (self.begin_segmet== other.begin_segmet) &&
        (self.end_segment == other.end_segment)
    }
}


fn tokenize_line<'a>(line_of_code :String, token_rules : &'a Vec<TokenRules>) -> Result<Vec<Token<'a>>,TokenError>
{
    if(line_of_code.len()<=0)
    {
        Err(TokenError::empty_line_of_code)
    }
    else 
    { 
        let mut tokens = vec![];
        let mut match_heap = BinaryHeap::new();
        token_rules
        .iter()
        .map(|token_match_rule|
        {
            match Regex::new(&token_match_rule.regex_rule)
            {
                
                Ok(regex) =>
                {
                    
                    let matched_rules = regex.find_iter(&line_of_code);
                    
                    matched_rules.map(|t|
                    {
                        match_heap.push(TokenMatch{
                            literal : String::from(t.as_str()),
                            begin_segmet : t.start().clone(),
                            end_segment : t.end().clone(),
                            rule : token_match_rule

                        });
                    }).collect::<Vec<_>>();
                     
                },
                Err(_)=>{();}

            };
        }).collect::<Vec<_>>();
        match match_heap.pop()
        {
            Some(first_match) =>
            {
                tokens.push(Token{
                    content : first_match.literal.clone(),
                    token_type : &first_match.rule.token_type

                });
                
                let mut mutable_line_of_code = line_of_code.clone();
                mutable_line_of_code.split_off(first_match.begin_segmet);
                let mut end_range = line_of_code.clone().split_off(first_match.end_segment);
                mutable_line_of_code.push_str(&end_range);
                match tokenize_line(mutable_line_of_code,token_rules)
                {
                    Ok(tokenized) =>
                    {
                        tokens.extend(tokenized);
                        Ok(tokens)
                    },
                    Err(_) =>
                    {
                        Ok(tokens)
                    }
                }

            },
            None=>
            {
                Err(TokenError::no_match)
            }
        }
    }
}


#[test]
fn test_tokenizer()
{
    let token_rules = vec![TokenRules{
        token_type : &TokenType::ending_terminator,
        regex_rule : String::from(r";")
    }];

    let tokens = tokenize_line(String::from(";;"), &token_rules).expect("expect tokens");
    assert_eq!(tokens.len(),2);

    let first_token = tokens.first().expect("expected token after first");
    assert_eq!(first_token.content,";");
    assert_eq!(first_token.token_type,&TokenType::ending_terminator);
}


