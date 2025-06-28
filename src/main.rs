mod input;
mod lines;
mod tokens;
mod html;

use std::fs;
use std::error::Error;

fn main() {
    let pmarkdown_from_file: String = fs::read_to_string("./test_input.txt").unwrap();
    let input: input::Input = input::Input{
        pmarkdown: pmarkdown_from_file
    };

    let mut classified_lines: lines::ClassifiedLines = lines::ClassifiedLines::get_dummy();

    lines::classify_lines(&input, &mut classified_lines);

    tokens::classify_tokens_for_classified_lines(&mut classified_lines);
    
    let mut actual_html: html::ActualHTML = html::ActualHTML::get_dummy();

    html::produce_html(&mut classified_lines, &mut actual_html);

    println!("{}", actual_html.html);
}