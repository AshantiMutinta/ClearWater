enum token_error
{
    empty_line_of_code
}

enum TokenType
{
    basic
}
struct Token<'a>
{
    content : &'a String,
    token_type : TokenType
}

fn tokenize_line<'a>(line_of_code : &'a String) -> Result<Vec<&'a Token>,token_error>
{
    if(line_of_code.len()<=0)
    {
        Err(token_error::empty_line_of_code)
    }
    else 
    {   
        Ok(vec![])
    }
}
