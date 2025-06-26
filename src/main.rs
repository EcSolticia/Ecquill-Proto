
enum LineType {
    H1,
    H2,
    H3,
    P
}

struct Line {
    text: String,
    line_type: LineType
}

fn classify_lines(lines: std::str::Split<'_, char>, line_tokens: &mut Vec<Line>) {
    // iterate over lines
    for line in lines {
        let first_char: char = line.chars().nth(0).unwrap();
        if first_char == '#' {
            
            line_tokens.push(
                Line{
                    text: line.to_string(),
                    line_type: LineType::H1
                }
            );

        } else {

            line_tokens.push(
                Line{
                    text: line.to_string(),
                    line_type: LineType::P
                }
            );
            
        }
    }
}

fn test_basic_types(line_tokens: &mut Vec<Line>) {
    for line in line_tokens {

        match line.line_type {
            LineType::H1 => println!("{}", line.text.to_uppercase()),
            LineType::P => println!("{}", line.text.to_lowercase()),
            _ => println!("[Not yet implemented :p]")
        }

    }
}

fn main() {
    let markdown: String = String::from("# hello\nbye\n# okay?")  ;

    let lines: std::str::Split<'_, char> = markdown.split('\n');
    
    let mut line_tokens: Vec<Line> = vec![];

    classify_lines(lines, &mut line_tokens);

    test_basic_types(&mut line_tokens);

}
