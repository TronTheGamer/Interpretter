use std::env;
use std::fs;
use std::io::{self, Write};


fn scan_token(file_contents: &str) {
    // Placeholder for the scanner implementation
    // This function will tokenize the input string and print tokens
    // For now, it just prints "EOF null" to indicate end of file
    for c in file_contents.chars(){
        match c {
            '(' => println!("LEFT_PAREN  ( null"),
            ')' => println!("RIGHT_PAREN  ) null"),
            _ => {},
        }
    }
    println!("EOF  null");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        /*
        * Match function to handle multiple commands.
        * The first command is "tokenize", which will be implemented in the next step.
        * Each command pattern is initalized as: "pattern" => {... code block for the command ...}
        * */
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            // writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();
            // scanToken(&filename);

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            // Uncomment this block to pass the first stage
            if !file_contents.is_empty() {
                // panic!("Scanner not implemented");
                scan_token(&file_contents);

            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
