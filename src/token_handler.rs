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
        let mut equal_index = 0;
        for c in file_contents.chars() {
            if c == '=' {
                    equal_index += 1;
                    if equal_index == 1 {
                        println!("EQUAL = null");
                    } else if equal_index == 2 {
                        equal_index = 0;
                        println!("EQUAL_EQUAL == null");
                    }
                } else{
                    equal_index = 0;
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
                        '\n' => {
                            line_number += 1;
                        },
                        _ => {
                            eprintln!("[line {line_number}] Error: Unexpected character: {c}");
                            has_error = true;
                        }
                    }
            }
        }

        println!("EOF  null");

        if has_error {
            exit(65);
        }
    }
}