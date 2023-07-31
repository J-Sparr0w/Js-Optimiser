use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Semicolon,
    Dot,
    Comma,
    Colon,
    LSquare,
    RSquare,
    LParen,
    RParen,
    LSquirly,
    RSquirly,
    DQuote,
    SQuote,
    BTick,
    Exp,
    Plus,
    Minus,
    Mul,
    Div,
    Not,
    And,
    Or,
    Question,
    LesserThan,
    GreaterThan,
    LessOrEqual,
    GreatOrEqual,
    LooseEqual,
    StrictEqual,
    NotEqual,
    StrictNotEqual,
    Arrow,
    Equals,
    Function,
    Const,
    Let,
    Var,
    Identifier(String),
    Keyword(String),
    Number(String),
    Invalid(String),
    EOF,
}

impl Token {
    pub fn to_str(&self) -> String {
        match self {
            Token::Semicolon => ";".to_string(),
            Token::Dot => ".".to_string(),
            Token::Comma => ",".to_string(),
            Token::Colon => ":".to_string(),
            Token::LSquare => "[".to_string(),
            Token::RSquare => "]".to_string(),
            Token::LParen => "(".to_string(),
            Token::RParen => ")".to_string(),
            Token::LSquirly => "{".to_string(),
            Token::RSquirly => "}".to_string(),
            Token::DQuote => "\"".to_string(),
            Token::SQuote => "'".to_string(),
            Token::BTick => "`".to_string(),
            Token::Exp => "^".to_string(),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Mul => "*".to_string(),
            Token::Div => "/".to_string(),
            Token::And => "&".to_string(),
            Token::Or => "|".to_string(),
            Token::Question => "?".to_string(),
            Token::Not => "!".to_string(),
            Token::NotEqual => "!=".to_string(),
            Token::StrictNotEqual => "!==".to_string(),
            Token::LesserThan => "<".to_string(),
            Token::LessOrEqual => "<=".to_string(),
            Token::GreaterThan => ">".to_string(),
            Token::GreatOrEqual => ">=".to_string(),
            Token::LooseEqual => "==".to_string(),
            Token::StrictEqual => "===".to_string(),
            Token::Arrow => "=>".to_string(),
            Token::Equals => "=".to_string(),
            Token::Function => "function ".to_string(),
            Token::Const => "const ".to_string(),
            Token::Let => "let ".to_string(),
            Token::Var => "var ".to_string(),
            Token::Identifier(ident) => ident.to_owned(),
            Token::Keyword(keyword) => keyword.to_owned() + " ",
            Token::Number(num_literal) => num_literal.to_owned(),
            _ => " ".to_string(),
        }
    }
}

pub struct Lexer {
    pub tokens: Vec<Token>,
    pub identifiers: HashMap<String, usize>,
    keywords: Vec<String>,
}

impl Lexer {
    pub fn new() -> Self {
        let file_contents = include_str!("../models/keywords.txt");

        let keywords: Vec<String> = file_contents
            .split_ascii_whitespace()
            .map(|slice| slice.to_string())
            .collect();

        Self {
            tokens: Vec::new(),
            identifiers: HashMap::new(),
            keywords,
        }
    }

    pub fn start(&mut self, text: String) {
        /*
        const student = "Raj";
        student = "Raju";
        //Just a comment
        var mark = 10;
        mark = 0;
                 */

        let mut char_iter = text.chars().peekable();

        while let Some(mut curr_char) = char_iter.next() {
            let mut lexeme = String::from(curr_char);
            let next_char = char_iter.peek().unwrap_or(&'\0');

            match (curr_char, &next_char) {
                //comments and white space
                (';', _) => self.tokens.push(Token::Semicolon),
                ('.', _) => self.tokens.push(Token::Dot),
                (',', _) => self.tokens.push(Token::Comma),
                (' ' | '\t' | '\n' | '\r', _) => continue,
                (':', _) => self.tokens.push(Token::Colon),
                ('[', _) => self.tokens.push(Token::LSquare),
                (']', _) => self.tokens.push(Token::RSquare),
                ('(', _) => self.tokens.push(Token::LParen),
                (')', _) => self.tokens.push(Token::RParen),
                ('{', _) => self.tokens.push(Token::LSquirly),
                ('}', _) => self.tokens.push(Token::RSquirly),
                ('"', _) => self.tokens.push(Token::DQuote),
                ('\'', _) => self.tokens.push(Token::SQuote),
                ('`', _) => self.tokens.push(Token::BTick),
                ('^', _) => self.tokens.push(Token::Exp),
                ('+', _) => self.tokens.push(Token::Plus),
                ('*', _) => self.tokens.push(Token::Mul),
                ('!', '=') => {
                    char_iter.next().unwrap();
                    if let Some(peek) = char_iter.peek() {
                        if *peek == '=' {
                            self.tokens.push(Token::StrictNotEqual);
                        } else {
                            self.tokens.push(Token::NotEqual);
                        }
                    }
                }
                ('!', _) => self.tokens.push(Token::Not),
                ('&', _) => self.tokens.push(Token::And),
                ('|', _) => self.tokens.push(Token::Or),
                ('?', _) => self.tokens.push(Token::Question),
                ('<', '=') => self.tokens.push(Token::LessOrEqual),
                ('<', _) => self.tokens.push(Token::LesserThan),
                ('>', '=') => self.tokens.push(Token::GreatOrEqual),
                ('>', _) => self.tokens.push(Token::GreaterThan),
                ('=', '=') => {
                    char_iter.next().unwrap();

                    if let Some(peek) = char_iter.peek() {
                        if *peek == '=' {
                            self.tokens.push(Token::StrictEqual);
                        } else {
                            self.tokens.push(Token::LooseEqual);
                        }
                    }
                }
                ('=', _) => {
                    if let Some(c) = char_iter.peek() {
                        if *c == '>' {
                            char_iter.next().unwrap();
                            self.tokens.push(Token::Arrow);
                        } else {
                            self.tokens.push(Token::Equals)
                        }
                    }
                }
                ('-', '0'..='9') | ('0'..='9', _) => {
                    //could be a float
                    let mut num_literal = String::new();

                    num_literal.push(curr_char);
                    while let Some(c) = char_iter.peek() {
                        if c.is_digit(16) || *c == '.' {
                            let next_char = char_iter.next().unwrap();
                            num_literal.push(next_char);
                        } else {
                            break;
                        }
                    }
                    self.tokens.push(Token::Number(num_literal));
                }
                ('-', _) => self.tokens.push(Token::Minus),
                ('/', _) => {
                    if let Some(c) = char_iter.peek() {
                        match *c {
                            '/' => {
                                while let Some(curr_char) = char_iter.next() {
                                    if curr_char == '\n' {
                                        break;
                                    }
                                    continue;
                                }
                            }
                            '*' => {
                                while let Some(curr_char) = char_iter.next() {
                                    if curr_char == '*' {
                                        if let Some(c) = char_iter.peek() {
                                            if *c == '/' {
                                                char_iter.next();
                                                break;
                                            }
                                        }
                                    }
                                    continue;
                                }
                            }
                            _ => self.tokens.push(Token::Div),
                        }
                    }
                }

                ('a'..='z' | 'A'..='Z' | '_' | '$', _) => {
                    //identifiers

                    while char_iter.peek().is_some()
                        && (char_iter.peek().unwrap().is_alphanumeric()
                            || *char_iter.peek().unwrap() == '_')
                    {
                        curr_char = char_iter.next().unwrap();
                        lexeme.push(curr_char);
                    }

                    if lexeme == "function" {
                        self.tokens.push(Token::Function);
                    } else if lexeme == "const" {
                        self.tokens.push(Token::Const);
                    } else if lexeme == "let" {
                        self.tokens.push(Token::Let);
                    } else if lexeme == "var" {
                        self.tokens.push(Token::Var);
                    } else if self.is_keyword(&lexeme) {
                        self.tokens.push(Token::Keyword(lexeme));
                    } else {
                        self.tokens.push(Token::Identifier(lexeme));
                    }
                }
                _ => {
                    self.tokens.push(Token::Invalid(lexeme));
                }
            }
            match self.tokens.last() {
                Some(Token::Identifier(ref ident) | Token::Keyword(ref ident)) => {
                    let len = self.tokens.len();
                    let curr_index = len - 1;
                    self.identifiers.insert(ident.to_string(), curr_index);
                }
                _ => (),
            }
        }
    }

    fn is_keyword(&self, lexeme: &str) -> bool {
        self.keywords.contains(&lexeme.to_owned())
    }
}

#[test]
fn equality_test() {
    assert_eq!(
        Token::Identifier("a".to_string()),
        Token::Identifier("a".to_string())
    );
    assert_eq!(
        Token::Keyword("const".to_string()),
        Token::Keyword("const".to_string())
    );
}
