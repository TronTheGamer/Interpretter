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

        let mut chars = file_contents.chars().peekable(); // Use a peekable iterator

        while let Some(c) = chars.next() {
            match prev_char {
                Some('=') => {
                    if c == '=' {
                        println!("EQUAL = null");
                        prev_char = None;
                        continue;
                    } else {
                        println!("EQUAL = null");
                    }
                }
                Some('!') => {
                    if c == '=' {
                        println!("BANG_EQUAL != null");
                        prev_char = None;
                        continue;
                    } else {
                        println!("BANG ! null");
                    }
                }
                Some('<') => {
                    if c == '=' {
                        println!("LESS_EQUAL <= null");
                        prev_char = None;
                        continue;
                    } else {
                        println!("LESS < null");
                    }
                }
                Some('>') => {
                    if c == '=' {
                        println!("GREATER_EQUAL >= null");
                        prev_char = None;
                        continue;
                    } else {
                        println!("GREATER > null");
                    }
                }
                                
                Some('/') => {
                    if c == '/' {
                        // Single-line comment, skip to the end of the line
                        while let Some(next_char) = chars.next() {
                            if next_char == '\n' {
                                break;
                            }
                        }
                        prev_char = None;
                        continue;
                    } else {
                        println!("SLASH / null");
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
                ';' => println!("SEMICOLON ; null"),
                '/' => {}
                '>' => {}
                '<' => {}
                '!' => {}
                '=' => {}
                '0'..='9' => println!("NUMBER {} {c}", c),
                '\n' => {
                    line_number += 1;
                }
                _ => {
                    eprintln!("[line {line_number}] Error: Unexpected character: {c}");
                    has_error = true;
                }
            }
        }

        if let Some('=') = prev_char {
            println!("EQUAL = null");
        }

        println!("EOF  null");

        if has_error {
            exit(65);
        }
    }
}