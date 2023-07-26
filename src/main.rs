use js_optimizer::{Lexer, Token};
use std::{fs::File, io::Read, path::Path};

enum LineType {
    Init,
    Assign,
    Unknown,
}

struct Line {
    line_no: u16,
    tokens: Vec<Token>,
    line_type: LineType,
}

impl Line {
    fn new() -> Self {
        Self {
            line_no: 1,
            tokens: Vec::new(),
            line_type: LineType::Unknown,
        }
    }
    fn set_type(&mut self, line_type: LineType) {
        self.line_type = line_type;
    }
    fn inc_line_no(&mut self) {
        self.line_no += 1;
    }
    fn set_line_no(&mut self, line_no: u16) {
        self.line_no = line_no;
    }
    fn push_token(&mut self, token: &Token) {
        self.tokens.push(token.clone());
    }
}
fn main() {
    let start = std::time::Instant::now();

    let args = std::env::args().collect::<Vec<String>>();

    let mut file = {
        let file_name: &String = &args[1];
        File::open(Path::new(&file_name)).unwrap()
    };

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let mut lexer = Lexer::new();
    lexer.start(buffer);

    let elapsed = std::time::Instant::now() - start;

    println!("{elapsed:?}");

    let mut lines: Vec<Line> = Vec::new();

    let mut line: Line = Line::new();
    for token in lexer.tokens.iter().peekable() {
        line.push_token(token);
        match token {
            Token::Keyword(k) => {
                if k == "const" || k == "let" || k == "var" {
                    line.set_type(LineType::Init);
                }
            }
            Token::Semicolon => {
                line.push_token(token);
                lines.push(line);
                line = Line::new();
            }
            _ => unimplemented!(),
        }
    }
}
