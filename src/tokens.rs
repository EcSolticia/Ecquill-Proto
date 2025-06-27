use crate::lines;
use regex::Regex;

pub enum TokenType {
    REGULAR,
    BOLD,
    ITALIC,
    HYPERLINK
}

pub struct Token {
    pub ttype: TokenType,
    pub ttext: String
}

pub struct ClassifiedTokens {
    pub tokens: Vec<Token>
}
impl ClassifiedTokens {
    pub fn get_dummy() -> ClassifiedTokens {
        return ClassifiedTokens{
            tokens: vec![]
        }
    }
}

pub fn classify_token(text_token: &str) -> TokenType {
    let bold_re = Regex::new(r"\*\*(.*?)\*\*").unwrap();
    let italic_re = Regex::new(r"\*(.*?)\*").unwrap();
    let hyperlink_re = Regex::new(r"\[(.*?)\]").unwrap();

    if bold_re.is_match(text_token) {
        return TokenType::BOLD;
    }
    if italic_re.is_match(text_token) {
        return TokenType::ITALIC;
    }
    if hyperlink_re.is_match(text_token) {
        return TokenType::HYPERLINK;
    }
    
    return TokenType::REGULAR;
}

pub fn classify_tokens(classified_line: &mut lines::Line) {
    let mut text_tokens = classified_line.ltext.split_whitespace();

    for text_token in text_tokens {


        classified_line.ltokens.tokens.push(Token{
            ttype: classify_token(text_token),
            ttext: text_token.to_string()
        })
    }
}

pub fn classify_tokens_for_classified_lines(classified_lines: &mut lines::ClassifiedLines) {
    for classified_line in &mut classified_lines.lines {
        classify_tokens(classified_line);
    }
}
