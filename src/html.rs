use crate::lines;
use crate::tokens;
use regex::Regex;

pub struct ActualHTML {
    pub html: String
}
impl ActualHTML {
    pub fn get_dummy() -> ActualHTML {
        return ActualHTML {html: "".to_string()};
    }
}

// no error handling
fn strip_tokenwise_pmarkdown_formatting(token: &tokens::Token) -> String {
    match token.ttype {
        tokens::TokenType::REGULAR => return token.ttext.clone(),
        tokens::TokenType::ITALIC => {
            let re = Regex::new(r"\*(.*?)\*").unwrap();
            return re.captures(&token.ttext).unwrap()[1].to_string();
        },
        tokens::TokenType::BOLD => {
            let re = Regex::new(r"\*\*(.*?)\*\*").unwrap();
            return re.captures(&token.ttext).unwrap()[1].to_string();
        },
        tokens::TokenType::HYPERLINK => {
            println!("Hyperlink match \"{}\"", token.ttext);
            let re = Regex::new(r"\[(.*?)\]").unwrap();
            return re.captures(&token.ttext).unwrap()[1].to_string();
        }
    }
}

fn strip_linewise_pmarkdown_formatting(line: &lines::Line) -> String {
    match line.ltype {
        lines::LineType::H => {
            let re = Regex::new(r"#(.*)").unwrap();
            let innerhtml: String = re.captures(&line.ltext).unwrap()[1].to_string();
            return format!("<h1>{}</h1>", innerhtml);
        },
        lines::LineType::P => {
            let re = Regex::new(r"(.*)").unwrap();
            let innerhtml: String = re.captures(&line.ltext).unwrap()[1].to_string();
            return format!("<p>{}</p>", innerhtml);
        }
        lines::LineType::NOTHING => return "".to_string()
    }
}

fn format_token_into_html(token: &tokens::Token) -> String {
    let mut chosen_tag: String = "".to_string();
    
    match token.ttype {
        tokens::TokenType::REGULAR => chosen_tag = chosen_tag,
        tokens::TokenType::BOLD => chosen_tag = "b".to_string(),
        tokens::TokenType::ITALIC => chosen_tag = "i".to_string(),
        tokens::TokenType::HYPERLINK => chosen_tag = "a".to_string()
    }

    if chosen_tag == "" {
        return format!("{} ", token.ttext.clone());
    }

    return format!("<{}>{} </{}>", chosen_tag, strip_tokenwise_pmarkdown_formatting(token), chosen_tag);
}

pub fn produce_html(
    classified_lines_with_classified_tokens: &mut lines::ClassifiedLines,
    actual_html: &mut ActualHTML
) {
    actual_html.html.push_str("<html><body>\n");
    for line in &mut classified_lines_with_classified_tokens.lines {

        line.ltext = "".to_string();

        for token in &line.ltokens.tokens {
            line.ltext.push_str(
                &format_token_into_html(token)
            )
        }

        let new_line: String = format!("{}\n", strip_linewise_pmarkdown_formatting(line));
        actual_html.html.push_str(&new_line);

    }
    actual_html.html.push_str("</html></body>");
}
