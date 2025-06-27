use crate::{input, tokens};

pub enum LineType {
    H,
    P,
    NOTHING
}

pub struct Line {
    pub ltype: LineType,
    pub ltext: String,
    pub ltokens: tokens::ClassifiedTokens
}

pub struct ClassifiedLines {
    pub lines: Vec<Line>
}

impl ClassifiedLines {
    pub fn get_dummy() -> ClassifiedLines {
        return ClassifiedLines {
            lines: vec![]
        }
    }
}

fn classify_line(line_text: &str) -> LineType {
    let mut splitlines: std::str::SplitWhitespace<'_> = line_text.split_whitespace();

    match splitlines.next() {
        Some("#") => return LineType::H,
        Some("[IGNORE]") => return LineType::NOTHING,
        _ => return LineType::P
    }
}

pub fn classify_lines(input: &input::Input, classified_lines: &mut ClassifiedLines) {
    let lines: std::str::Split<'_, char> = input.pmarkdown.split('\n');

    for line_text in lines {

        let line_type: LineType = classify_line(line_text);

        match line_type {
            LineType::NOTHING => continue,
            _ => classified_lines.lines.push(Line{
                ltype: line_type,
                ltext: line_text.to_string(),
                ltokens: tokens::ClassifiedTokens::get_dummy()
            })
        }

    }
}
