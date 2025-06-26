
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

fn to_html(line_tokens: &mut Vec<Line>, output: &mut String) {
    for line in line_tokens {

        match line.line_type {

            LineType::H1 => output.push_str(format!("<h1>{}</h1>", line.text.as_str()[1..].to_string()).as_str()),
            LineType::H2 => output.push_str(format!("<h2>{}</h2>", line.text.as_str()[2..].to_string()).as_str()),//noimpl
            LineType::H3 => output.push_str(format!("<h3>{}</h3>", line.text.as_str()[3..].to_string()).as_str()),//noimpl
            LineType::P => output.push_str(format!("<p>{}</p>", line.text).as_str())

        }

    }
}

fn main() {
    let markdown: String = String::from("# hello\nbye\n# okay?")  ;

    let lines: std::str::Split<'_, char> = markdown.split('\n');
    
    let mut line_tokens: Vec<Line> = vec![];

    classify_lines(lines, &mut line_tokens);

    let mut output: String = "".to_string();

    to_html(&mut line_tokens, &mut output);;

    println!("{}", output);

}
