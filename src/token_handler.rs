use std::process::exit;

/// A struct to handle tokenization logic.
pub struct TokenHandler;

impl TokenHandler {


    /// Tokenizes the input string and prints tokens to the console.
    ///
    /// # Arguments
    /// * `file_contents` - A string slice containing the contents of the file to be tokenized.
    ///
    /// # Behavior
    /// This function scans through each character in the input string and matches it against
    /// specific token patterns (e.g., parentheses, braces). For each match, it prints the token
    /// type and value. At the end of the input, it prints "EOF null" to indicate the end of the file.
    pub fn scan_token(&self, file_contents: &str) {
    let mut line_number = 1;
    let mut has_error = false;
    let mut prev_char: Option<char> = None; // Track the previous character

    for c in file_contents.chars() {
        // if let Some('=') = prev_char {
        //     if c == '=' {
        //         println!("EQUAL_EQUAL == null");
        //         prev_char = None; // Reset after handling "=="
        //         continue;
        //     } else {
        //         println!("EQUAL = null");
        //     
        // }

        match prev_char {
            Some('=') => {
                // Handle the case where the previous character was '='
                if c == '=' {
                    println!("EQUAL_EQUAL == null");
                    prev_char = None; // Reset after handling "=="
                    continue;
                } else {
                    println!("EQUAL = null");
                }
            }
            Some('!') => {
                // Handle the case where the previous character was '!'
                if c == '=' {
                    println!("BANG_EQUAL != null");
                    prev_char = None; // Reset after handling "!="
                    continue;
                } else {
                    println!("BANG ! null");
                }
            }
            _ => {}
        }

        prev_char = Some(c);

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
            '!' => {}
            '0'..='9' => println!("NUMBER {} {c}", c),
            '\n' => {
                line_number += 1;
            },
            '=' => {}
            _ => {
                eprintln!("[line {line_number}] Error: Unexpected character: {c}");
                has_error = true;
            }
        }
    }

    // Handle a trailing "=" if it exists
    if let Some('=') = prev_char {
        println!("EQUAL = null");
    }

    println!("EOF  null");

    if has_error {
        exit(65);
    }
}
}