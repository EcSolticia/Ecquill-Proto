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
fn strip_pmarkdown_formatting(token: &tokens::Token) -> String {
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

    return format!("<{}>{} </{}>", chosen_tag, strip_pmarkdown_formatting(token), chosen_tag);
}

pub fn produce_html(
    classified_lines_with_classified_tokens: &lines::ClassifiedLines,
    actual_html: &mut ActualHTML
) {
    for line in &classified_lines_with_classified_tokens.lines {

        let mut hline: String = "<html><body>\n".to_string();
        let mut chosen_tag: String = "".to_string();

        match line.ltype {
            lines::LineType::H => chosen_tag = "h1".to_string(),
            lines::LineType::P => chosen_tag = "p".to_string(),
            lines::LineType::NOTHING => continue
        }

        hline.push_str(&format!("<{}>", chosen_tag));

        for token in &line.ltokens.tokens {
            hline.push_str(
                &format_token_into_html(token)
            );
        }

        hline.push_str(&format!("</{}>\n", chosen_tag));

        actual_html.html.push_str(&hline);
        actual_html.html.push_str("</body></html>")
    }
}
