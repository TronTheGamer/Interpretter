use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

/// Scanner function to tokenize the input string
/// # Arguments
/// * `file_contents` - A string slice that holds the contents of the file to be tokenized
/// # Behavior
/// This function will scan the input string and print tokens to the standard output.
/// It will print "EOF null" to indicate the end of the file.
/// Prints in the following format:
/// * '<TOKEN_TYPE> <LEXEM> <LITERAL>'
/// * `LEFT_PAREN ( null`
fn scan_token(file_contents: &str) {
    // Placeholder for the scanner implementation
    // This function will tokenize the input string and print tokens
    // For now, it just prints "EOF null" to indicate end of file
    let mut line_number = 1;
    let mut has_error = false;
    for c in file_contents.chars(){
        match c {
            '(' => println!("LEFT_PAREN ( null"),
            ')' => println!("RIGHT_PAREN ) null"),
            '{' => println!("LEFT_BRACE {{ null"),
            '}' => println!("RIGHT_BRACE }} null"),
            '*' => println!("STAR * null"),
            '+' => println!("PLUS + null"),
            '-' => println!("MINUS - null"),
            ',' => println!("COMMA , null"),
            '.' => println!("DOT . null"),
            '/' => println!("SLASH / null"),
            ';' => println!("SEMICOLON ; null"),
            '0'..='9' => println!("NUMBER {} {c}", c),
            '\n' => {line_number += 1;},
            _ => {
                eprintln!("[line {line_number}] Error: Unexpected character: {c}");
                exit(65);
            },
        }
    }
    println!("EOF null");
    if(has_error){
        exit(65);
    }
}

/// Entry point of the program.
///
/// This function parses command-line arguments and executes the appropriate command.
/// Currently, it supports the "tokenize" command, which reads a file and tokenizes its contents.
///
/// # Usage
///
/// ```bash
/// <program_name> tokenize <filename>
/// ```
///
/// # Behavior
///
/// If the "tokenize" command is provided with a valid filename, the program reads the file
/// and passes its contents to the `scan_token` function for tokenization. If the command
/// or filename is invalid, an error message is printed to `stderr`.
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
