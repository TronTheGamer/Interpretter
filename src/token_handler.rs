use std::process::exit;
use std::iter::Peekable;
use std::str::Chars;

/// A struct to handle tokenization logic.
pub struct TokenHandler {
    pub line_number: i32,
    pub has_error: bool,
    tokens: Vec<String>,
}

impl Default for TokenHandler {
    fn default() -> Self {
        TokenHandler {
            line_number: 1,
            has_error: false,
             tokens: Vec::new(),
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
                        self.add_token("EQUAL_EQUAL == null".to_string());
                        prev_char = None;
                        continue;
                    } else {
                        println!("EQUAL = null");
                        self.add_token("EQUAL = null".to_string());
                    }
                }
                Some('!') => {
                    if c == '=' {
                        println!("BANG_EQUAL != null");
                        self.add_token("BANG_EQUAL != null".to_string());
                        prev_char = None;
                        continue;
                    } else {
                        println!("BANG ! null");
                        self.add_token("BANG ! null".to_string());
                    }
                }
                Some('<') => {
                    if c == '=' {
                        println!("LESS_EQUAL <= null");
                        self.add_token("LESS_EQUAL <= null".to_string());
                        prev_char = None;
                        continue;
                    } else {
                        println!("LESS < null");
                        self.add_token("LESS < null".to_string());
                    }
                }
                Some('>') => {
                    if c == '=' {
                        println!("GREATER_EQUAL >= null");
                        self.add_token("GREATER_EQUAL >= null".to_string());
                        prev_char = None;
                        continue;
                    } else {
                        println!("GREATER > null");
                        self.add_token("GREATER > null".to_string());
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
                        self.add_token("SLASH / null".to_string());
                    }
                }
                _ => {}
            }

            prev_char = Some(c);

            match c {
                '(' => {println!("LEFT_PAREN ( null"); self.add_token("LEFT_PAREN ( null".to_string());},
                ')' => {println!("RIGHT_PAREN ) null"); self.add_token("RIGHT_PAREN ) null".to_string());},
                '{' => {println!("LEFT_BRACE {{ null"); self.add_token("LEFT_BRACE {{ null".to_string());},
                '}' => {println!("RIGHT_BRACE }} null"); self.add_token("RIGHT_BRACE }} null".to_string());},
                '*' => {println!("STAR * null"); self.add_token("STAR * null".to_string());},
                '+' => {println!("PLUS + null"); self.add_token("PLUS + null".to_string());},
                '-' => {println!("MINUS - null"); self.add_token("MINUS - null".to_string());},
                ',' => {println!("COMMA , null"); self.add_token("COMMA , null".to_string());},
                '.' => {println!("DOT . null"); self.add_token("DOT . null".to_string());},
                ';' => {println!("SEMICOLON ; null"); self.add_token("SEMICOLON ; null".to_string());},
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
                'a'..='z' | 'A'..='Z' | '_' => {
                    let identifier = self.indentifier_handle(&mut chars, c);
                }
                '\n' => {
                    self.line_number += 1;
                }
                ' ' | '\r' | '\t' => {}

                '"' => {
                    let str_literal: Option<String> = self.str_handle(&mut chars);
                    if let Some(literal) = str_literal {
                        println!("STRING \"{}\" {}", literal, literal);
                        self.add_token(format!("STRING \"{}\" {}", literal, literal));
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
            self.add_token("SLASH / null".to_string());
        } else if let Some('=') = prev_char {
            println!("EQUAL = null");
            self.add_token("EQUAL = null".to_string());
        } else if let Some('<') = prev_char {
            println!("LESS < null");
            self.add_token("LESS < null".to_string());
        } else if let Some('>') = prev_char {
            println!("GREATER > null");
            self.add_token("GREATER > null".to_string());
        } else if let Some('!') = prev_char {
            println!("BANG ! null");
            self.add_token("BANG ! null".to_string());
        }
        
        println!("EOF  null");

        if self.has_error {
            exit(65);
        }
    }

    fn str_handle(&mut self, chars: &mut Peekable<Chars>)->Option<String> {
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

    fn num_handle(&mut self, chars: &mut Peekable<Chars>, c: char) -> (String, f64) {

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

        if num_value.fract() == 0.0 {
            let _token = format!("NUMBER {} {:.1}", num_str, num_value);
            self.add_token(_token);
        } else {
            let _token = format!("NUMBER {} {}", num_str, num_value);
            self.add_token(_token);
        }     

        return (num_str, num_value);   
             
    }

    fn add_token(&mut self, token: String) {
        // Adds a token to the tokens vector.
        self.tokens.push(token);
    }

    pub fn get_tokens(&self) -> &Vec<String> {
        // Returns a reference to the tokens vector.
        return &self.tokens
    }

    fn indentifier_handle(&mut self, chars: &mut Peekable<Chars>, c: char) -> String {
        let mut identifier: String = String::new();
        identifier.push(c);
        while let Some(next_char) = chars.peek() {
            if next_char.is_alphanumeric() || *next_char == '_' {
                identifier.push(*next_char);
                chars.next();
            } else {
                break;
            }
        }

        let _token: String = if self.is_keyword(&identifier) {
            format!("KEYWORD {} null", identifier)
        } else {
            format!("IDENTIFIER {} null", identifier)
        };
        self.add_token(_token.clone());

        println!("{}",_token.clone());

        return identifier;
    }
    fn is_keyword(&self, identifier: &str) -> bool {
        // Check if the identifier is a keyword
        match identifier {
            "and" | "class" | "else" | "false" | "for" | "fun" | "if" | "nil" | "or" | "print" |
            "return" | "super" | "this" | "true" | "var" | "while" => true,
            _ => false,
        }
    }


}