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
    Lesser,
    Greater,
    Equals,
    Identifier(String),
    Keyword(String),
    Number(isize),
    Invalid,
}

// impl PartialEq for Token {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Token::Identifier(a), Token::Identifier(b)) => a == b,
//             (Token::Keyword(a), Token::Keyword(b)) => a == b,
//             _ => self == other,
//         }
//     }
// }

pub struct Lexer {
    pub tokens: Vec<Token>,
    keywords: Vec<String>,
}

impl Lexer {
    pub fn new() -> Self {
        let file_contents = include_str!("models/keywords.txt");

        let keywords: Vec<String> = file_contents
            .split_ascii_whitespace()
            .map(|slice| slice.to_string())
            .collect();

        Self {
            tokens: Vec::new(),
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
            match curr_char {
                //comments and white space
                ';' => self.tokens.push(Token::Semicolon),
                '.' => self.tokens.push(Token::Dot),
                ',' => self.tokens.push(Token::Comma),
                ' ' | '\t' | '\n' | '\r' => continue,
                ':' => self.tokens.push(Token::Colon),
                '[' => self.tokens.push(Token::LSquare),
                ']' => self.tokens.push(Token::RSquare),
                '(' => self.tokens.push(Token::LParen),
                ')' => self.tokens.push(Token::RParen),
                '{' => self.tokens.push(Token::LSquirly),
                '}' => self.tokens.push(Token::RSquirly),
                '"' => self.tokens.push(Token::DQuote),
                '\'' => self.tokens.push(Token::SQuote),
                '`' => self.tokens.push(Token::BTick),
                '^' => self.tokens.push(Token::Exp),
                '+' => self.tokens.push(Token::Plus),
                '-' => self.tokens.push(Token::Minus),
                '*' => self.tokens.push(Token::Mul),
                '!' => self.tokens.push(Token::Not),
                '&' => self.tokens.push(Token::And),
                '|' => self.tokens.push(Token::Or),
                '?' => self.tokens.push(Token::Question),
                '<' => self.tokens.push(Token::Lesser),
                '>' => self.tokens.push(Token::Greater),
                '=' => self.tokens.push(Token::Equals),
                '/' => {
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
                _ => {
                    //identifiers
                    if curr_char.is_alphabetic()
                        || curr_char.to_owned() == '_'
                        || curr_char.to_owned() == '$'
                    {
                        while char_iter.peek().is_some()
                            && char_iter.peek().unwrap().is_alphanumeric()
                        {
                            curr_char = char_iter.next().unwrap();
                            lexeme.push(curr_char);
                        }
                        if self.is_keyword(&lexeme) {
                            self.tokens.push(Token::Keyword(lexeme));
                        } else {
                            self.tokens.push(Token::Identifier(lexeme));
                        }
                    } else {
                        self.tokens.push(Token::Invalid);
                    }
                }
            }
        }

        //if char is Semicolon? add text till char to line
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
