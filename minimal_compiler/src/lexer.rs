use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Token {
    KEYWORD,
    IDENTIFIER,
    OPENPARENTHESIS,
    CLOSEPARENTHESIS,
    OPENBRACE,
    CONSTANT,
    SEMICOLON,
    CLOSEBRACE,
    UNKNOWN,
}

fn regex_resolver(token: &Token) -> Regex{
    let keyword = Regex::new(r"^(int|void|return)\b").unwrap();
    let identifier = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap();
    let openparenthesis = Regex::new(r"^\(").unwrap();
    let closeparenthesis = Regex::new(r"^\)").unwrap();
    let openbrace = Regex::new(r"^\{").unwrap();
    let constant = Regex::new(r"^[0-9]+\b").unwrap();
    let semicolon = Regex::new(r"^;").unwrap();
    let closebrace = Regex::new(r"^\}").unwrap();
    match token {
        Token::KEYWORD => return keyword,
        // If the token is an identifier, we need to check if it's a keyword first 
        Token::IDENTIFIER => {
            if token == &Token::KEYWORD {
                return keyword;
            }
            return identifier
        },
        Token::OPENPARENTHESIS => return openparenthesis,
        Token::CLOSEPARENTHESIS => return closeparenthesis,
        Token::OPENBRACE => return openbrace,
        Token::CONSTANT => return constant,
        Token::SEMICOLON => return semicolon,
        Token::CLOSEBRACE => return closebrace,
        Token::UNKNOWN => panic!("Unknown token"),
        
    }

}
fn token_vec() -> Vec<Token> {
    vec![
        Token::KEYWORD,
        Token::IDENTIFIER,
        Token::OPENPARENTHESIS,
        Token::CLOSEPARENTHESIS,
        Token::OPENBRACE,
        Token::CONSTANT,
        Token::SEMICOLON,
        Token::CLOSEBRACE,
    ]
}
fn token_resolver(input: &str) -> Token {
    match input {
        "int" | "void" | "return" => return Token::KEYWORD,
        "(" => return Token::OPENPARENTHESIS,
        ")" => return Token::CLOSEPARENTHESIS,
        "{" => return Token::OPENBRACE,
        "}" => return Token::CLOSEBRACE,
        ";" => return Token::SEMICOLON,
        _ => {
            let identifier_regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
            let constant_regex = Regex::new(r"^[0-9]+$").unwrap();
            if identifier_regex.is_match(input) {
                return Token::IDENTIFIER;
            } else if constant_regex.is_match(input) {
                return Token::CONSTANT;
            } else {
                return Token::UNKNOWN;
            }
        }
    }
}
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current = input;
    while !current.is_empty() {
        let mut matched = false;
        for token_type in token_vec().iter()
        {   
            //get the regex for the token type
            let regex = regex_resolver(token_type);
            //use the regex to make a search to get a match of the current input
            if let Some(mat) = regex.find(current) {          
                // after gettting the match, extract the token string from the current input
                let token_str = &current[mat.start()..mat.end()];
                let token = token_resolver(token_str);
                tokens.push(token);
                current = &current[mat.end()..].trim_start();
                matched = true;
                break;
            }
        }
        if !matched {
            panic!("Unknown token in input: {}", current);
        }
    }
    tokens
}

#[test]
fn test_tokenize() {
    let input = "int main(void) { return 2; }";
    let tokens = tokenize(input);
    let expected_tokens = vec![
        Token::KEYWORD,
        Token::IDENTIFIER,
        Token::OPENPARENTHESIS,
        Token::KEYWORD,
        Token::CLOSEPARENTHESIS,
        Token::OPENBRACE,
        Token::KEYWORD,
        Token::CONSTANT,
        Token::SEMICOLON,
        Token::CLOSEBRACE,
    ];
    assert_eq!(tokens, expected_tokens);
}
#[test]
fn test_token_resolver() {
    assert_eq!(token_resolver("int"), Token::KEYWORD);
    assert_eq!(token_resolver("main"), Token::IDENTIFIER);
    assert_eq!(token_resolver("("), Token::OPENPARENTHESIS);
    assert_eq!(token_resolver(")"), Token::CLOSEPARENTHESIS);
    assert_eq!(token_resolver("{"), Token::OPENBRACE);
    assert_eq!(token_resolver("2"), Token::CONSTANT);
    assert_eq!(token_resolver(";"), Token::SEMICOLON);
    assert_eq!(token_resolver("}"), Token::CLOSEBRACE);
}
#[test]
fn test_fail_identifier() {
    let input = "int 123main(void) { return 2; }";
    let result = std::panic::catch_unwind(|| {
        tokenize(input);
    });
    assert!(result.is_err());
}

#[test]
fn test_mix_up_identifier_and_keyword() {
    let input = "int intmain(void) { return 2; }";
    let tokens = tokenize(input);
    let expected_tokens = vec![
        Token::KEYWORD,
        Token::IDENTIFIER,
        Token::OPENPARENTHESIS,
        Token::KEYWORD,
        Token::CLOSEPARENTHESIS,
        Token::OPENBRACE,
        Token::KEYWORD,
        Token::CONSTANT,
        Token::SEMICOLON,
        Token::CLOSEBRACE,
    ];
    assert_eq!(tokens, expected_tokens);
}