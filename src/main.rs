
fn main() {
    let markdown: String = String::from("# hello\nbye\n# okay?")  ;

    let lines= markdown.split('\n');
    
    // iterate over lines
    for line in lines {
        let first_char: char = line.chars().nth(0).unwrap();
        if first_char == '#' {
            println!("{}", line.to_uppercase());
        } else {
            println!("{}", line);
        }
    }

}
