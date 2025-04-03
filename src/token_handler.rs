use std::process::exit;
use std::iter::Peekable;
use std::str::Chars;

/// A struct to handle tokenization logic.
pub struct TokenHandler {
    pub line_number: i32,
    pub has_error: bool,
}

impl Default for TokenHandler {
    fn default() -> Self {
        TokenHandler {
            line_number: 1,
            has_error: false,
        }
    }
}


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
    pub fn scan_token(&mut self, file_contents: &str) {
        self.line_number = 1;
        self.has_error = false;
        let mut prev_char: Option<char> = None; // Track the previous character

        let mut chars = file_contents.chars().peekable(); // Use a peekable iterator

        while let Some(c) = chars.next() {
            match prev_char {
                Some('=') => {
                    if c == '=' {
                        println!("EQUAL_EQUAL == null");
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
                                self.line_number += 1;
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
                '/' => {
                    prev_char = Some('/');
                }
                '>' => {
                    prev_char = Some('>');
                }
                '<' => {
                    prev_char = Some('<');
                }
                '!' => {
                    prev_char = Some('!');
                }
                '=' => {}
                '0'..='9' => {
                    let (num_str, num_value) = self.num_handle(&mut chars, c);
                    if num_value.fract() == 0.0 {
                        println!("NUMBER {} {:.1}", num_str, num_value);
                    } else {
                        println!("NUMBER {} {}", num_str, num_value);
                    }
                }
                '\n' => {
                    self.line_number += 1;
                }
                ' ' | '\r' | '\t' => {}

                '"' => {
                    let str_literal: Option<String> = self.str_handle(&mut chars);
                    if let Some(literal) = str_literal {
                        println!("STRING \"{}\" {}", literal, literal);
                    } else {
                        // Handle error in string literal
                        eprintln!("[line {}] Error: Unterminated string.", self.line_number);
                        self.has_error = true;
                    }
                }

                _ => {
                    eprintln!("[line {}] Error: Unexpected character: {c}",self.line_number);
                    self.has_error = true;
                }
            }
        }

        // Handle any remaining character in prev_char
        if let Some('/') = prev_char {
            println!("SLASH / null");
        } else if let Some('=') = prev_char {
            println!("EQUAL = null");
        } else if let Some('<') = prev_char {
            println!("LESS < null");
        } else if let Some('>') = prev_char {
            println!("GREATER > null");
        } else if let Some('!') = prev_char {
            println!("BANG ! null");
        }

        println!("EOF  null");

        if self.has_error {
            exit(65);
        }
    }

    pub fn str_handle(&mut self, chars: &mut Peekable<Chars>)->Option<String> {
        let mut string_literal: String = String::new();
        while let Some(next_char) = chars.next() {
            match next_char {
                '"' => {
                    return Some(string_literal);
                }
                '\\' => {
                    if let Some(escaped_char) = chars.next() {
                        match escaped_char {
                            'n' => string_literal.push('\n'),
                            't' => string_literal.push('\t'),
                            '\\' => string_literal.push('\\'),
                            '"' => string_literal.push('"'),
                            _ => string_literal.push(escaped_char),
                        }
                    }
                    continue;
                }
                '\n' => {
                    self.has_error = true;
                    return None;
                }
                _ => {
                    string_literal.push(next_char);
                }
            }
            
    }
    return None;
    }

    pub fn num_handle(&mut self, chars: &mut Peekable<Chars>, c: char) -> (String, f64) {

        let mut num_str: String = String::new();
        num_str.push(c);
        while let Some(next_char) = chars.peek() {
            if next_char.is_digit(10) || *next_char == '.' {
                num_str.push(*next_char);
                chars.next();
            } else {
                break;
            }
        }

        let num_value: f64 = num_str.parse().unwrap_or(0.0);     

        return (num_str, num_value);   
             
    }


}