use std::io::prelude::*;
use std::fs::File;

const FILE_PATH : &str = "src/main.fs";

#[derive(PartialEq)]
enum Token {
    DIVIDE,
    ADD,
    SUBTRACT,
    PRINT,
    FUNCTION(String, Vec<Token>),
    NUMBER(i32),
    STRING(String),
    IF (Vec<Token>),
    END,
    CALL_FUNCTION(String),
}

struct Reader  {
    pos: usize,
    len: usize,
    data: String
}
impl Reader {
    fn new (path : &str) -> Self {
        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Error reading file");
        // end on a whitespace, makes parsing easier
//        data.push(' ');
        Self { pos: 0, len: data.len(), data: data }
    }

    fn next (&mut self) {
        self.pos += 1;
    }
//    fn back (&mut self) {
//        self.pos -= 1;
//    }
    fn peek (&self) -> char {
        return self.data.chars().nth(self.pos).expect("PEEKED PAST POSSIBLE");
    }
    fn is_whitespace (&self) -> bool {
        if self.peek() == ' ' || self.peek() == '\n' || self.peek() == '\t' {
            return true;
        }
        return false;
    }
    fn at_end (&self) -> bool {
        if self.pos < self.len {
            return false;
        }
        return true;
    }
}



fn print_tokens (tokens: Vec<Token>) {
    for token in tokens {
        match token {
            Token::NUMBER(n) => println!("NUMBER {}", n),
            Token::STRING(s) => println!("STRING \"{}\"", s),
            Token::DIVIDE => println!("/"),
            Token::ADD => println!("+"),
            Token::SUBTRACT => println!("-"),
            Token::IF(t) => {println!(" if < "); print_tokens(t); println!(" end > ");},
            Token::FUNCTION(n, t) => { println!("FUNCTION {} < ", n); print_tokens(t); println!(" endfunc > ")},
            Token::END => println!(" end > "),
            Token::PRINT => println!("PRINT"),
            Token::CALL_FUNCTION(s) => println!("CALL {}", s),
            _ => println!("?")
        }
    }
}


struct Tokenizer {
    r: Reader,
    functions: Vec<String>,
}
impl Tokenizer {
    fn new (file_path: &str) -> Tokenizer {
        Self {r: Reader::new(file_path), functions: Vec::new()}
    }


    fn get_tokens (&mut self, end: bool) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();
        while !self.r.at_end() {
            if self.r.peek() >= '0' && self.r.peek () <= '9' {
                tokens.push(self.read_number());
            }
            else if self.r.peek() == '\"' {
                tokens.push(self.read_string());
            } else if self.r.peek() == '/' {
                tokens.push(Token::DIVIDE);
            } else if self.r.peek() == '-' {
                tokens.push(Token::SUBTRACT);
            } else if self.r.peek() == '+' {
                tokens.push(Token::ADD);
            } else if self.r.is_whitespace() {}
            else {
                let tok = self.get_command();
                match tok {
                    Ok(x) => {
                        if x == Token::END {
                            if end {
                                return Ok(tokens);
                            } else {
                                return Err("END called in no block".to_string())
                            }
                        }
                        tokens.push(x);
                    }
                    Err(x) => return Err(x),
                }
            }
            self.r.next();
        }
        return Ok(tokens);
    }

    fn read_number (&mut self) -> Token {
        let mut number = String::new();
        while self.r.peek() >= '0' && self.r.peek() <= '9' && !self.r.at_end() {
            number.push(self.r.peek());
            self.r.next();
        }
        return Token::NUMBER(number.parse::<i32>().expect("ERROR WHILE PARSING NUMBER"));
    }

    fn read_string (&mut self) -> Token {
        let mut string = String::new();
        self.r.next();
        while self.r.peek() != '\"' && !self.r.at_end() {
            string.push(self.r.peek());
            self.r.next();
        }
        self.r.next();
        return Token::STRING(string);
    }

    fn valid_function (&mut self, func: String) -> bool {
        if self.functions.contains(&func) {
            return true;
        }
        return false;
    }

    fn get_command (&mut self) -> Result<Token, String> {
        let s = self.get_next_word();
        return match &s as &str {
            "print" => Ok(Token::PRINT),
            "if" => {
                match self.get_tokens(true) {
                    Ok(t) => Ok(Token::IF(t)),
                    Err(e) => Err(e),
                }
            },
            "end" => Ok(Token::END),
            "def" => {
                let name = self.get_next_word();
                match self.get_tokens(true) {
                    Ok(t) => {
                        self.functions.push(String::from(&name));
                        Ok(Token::FUNCTION(name, t))
                    },
                    Err(e) => Err(e),
                }
            },
            _ => {
                if self.valid_function(String::from(&s)) {
                    return Ok(Token::CALL_FUNCTION(String::from(&s)));
                }
                return Err("Unknwon call".to_string());
            },
        }
    }


    fn get_next_word (&mut self) -> String {
        while !self.r.at_end() && self.r.is_whitespace() {
            self.r.next();
        }
        let mut s = String::new();
        while !self.r.at_end() && !self.r.is_whitespace() {
            s.push(self.r.peek());
            self.r.next();
        }
        return s;
    }
}

fn main() {
    let mut tokenizer = Tokenizer::new(FILE_PATH);
    print_tokens(tokenizer.get_tokens(false).unwrap());
}
