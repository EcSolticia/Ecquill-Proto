mod input;
mod lines;
mod tokens;
mod html;

fn main() {
    let input: input::Input = input::Input{
        pmarkdown: "# Hi, this is **me**, okay?\nOk\n# Bye".to_string()
    };

    let mut classified_lines: lines::ClassifiedLines = lines::ClassifiedLines::get_dummy();

    lines::classify_lines(&input, &mut classified_lines);

    tokens::classify_tokens_for_classified_lines(&mut classified_lines);
    
    let mut actual_html: html::ActualHTML = html::ActualHTML::get_dummy();

    html::produce_html(&classified_lines, &mut actual_html);

    println!("{}", actual_html.html);
}