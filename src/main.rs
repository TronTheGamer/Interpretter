use std::env;
use std::fs;
use std::io::{self, Write};

// -------------------Imports-------------------------//
mod token_handler;
use token_handler::TokenHandler;


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
    let mut handler = TokenHandler {..Default::default()};

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
                handler.scan_token(&file_contents);

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
