mod io;
mod lines;

fn main() {
    let input: io::Input = io::Input{
        pmarkdown: "# Hi\nOk\n# Bye".to_string()
    };

    //let mut output: io::Output;

    let mut classified_lines: lines::ClassifiedLines = lines::ClassifiedLines::get_dummy();

    lines::classify_lines(&input, &mut classified_lines);

    for line in classified_lines.lines {

        match line.ltype {
            lines::LineType::H => println!("{}", line.ltext.to_uppercase()),
            lines::LineType::P => println!("{}", line.ltext.to_lowercase()),
            lines::LineType::NOTHING => continue
        }

    } 
}