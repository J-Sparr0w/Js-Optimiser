use std::option::Iter;

use crate::lexer::Lexer;

use super::lexer::Token;

#[derive(Debug)]
enum Scope {
    Global,
    Block(i32),
    Function(String),
}

#[derive(Debug)]
pub enum StatementKind {
    FunctionDecl,
    Declaration,
    Mutation,
    Initialization,
    Other,
    Unknown,
}

#[derive(Debug)]
pub struct Statement {
    line_no: u32,
    kind: StatementKind,
    tokens: Vec<Token>,
}

impl Statement {
    pub fn new() -> Self {
        Self {
            line_no: 1,
            kind: StatementKind::Unknown,
            tokens: Vec::new(),
        }
    }
    fn new_line(line_no: u32) -> Self {
        Self {
            line_no,
            kind: StatementKind::Unknown,
            tokens: Vec::new(),
        }
    }
    fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }
}

/*
identifiers     kind                                  declared_at   used_at         mutated_at
a               Function{params,return}               15,21,31        none
b               Variable                              10            20,30,40        20
*/

#[derive(Debug)]
enum IdentKind {
    Function { params: String, ret_val: String },
    Variable,
}

#[derive(Debug)]
enum Value {
    Variable(Box<Identifier>),
    VarExpression(String),
    ConstExpression(String),
    Constant(String),
    Undefined,
}

#[derive(Debug)]
struct TokenPos(u32, u32);

#[derive(Debug)]
struct Identifier {
    name: String,
    value: Value,
    kind: IdentKind,
    declared_at: TokenPos,
    used_at: Option<TokenPos>,
    mutated_at: Option<TokenPos>,
}

#[derive(Debug)]
struct Block {
    start: u32,
    end: u32,
    kind: Scope,
}

impl Block {
    pub fn new(start: u32) -> Self {
        Self {
            start,
            end: start,
            kind: Scope::Global,
        }
    }

    fn end_block(&mut self, line_no: u32) {
        self.end = line_no;
    }
}

#[derive(Debug)]
pub struct Parser {
    sym_tab: Vec<Identifier>,
    blocks: Vec<Block>,
    lines: Vec<Statement>,
    curr_index: u32,
    curr_line_no: u32,
    curr_scope: Scope,
}

impl Parser {
    pub fn parse_from_lexer(lexer: &Lexer) -> Self {
        let mut parser = Self {
            sym_tab: Vec::new(),
            blocks: Vec::new(),
            lines: Vec::new(),
            curr_index: 0,
            curr_line_no: 1,
            curr_scope: Scope::Global,
        };

        // TODO: use windows

        let mut curr_line: Statement = Statement::new();
        let mut block = Block::new(1);

        let mut peekable_iter = lexer.tokens.iter().peekable();
        while let Some(_) = peekable_iter.peek() {
            let token = peekable_iter.next().unwrap();
            parser.inc_curr_index();
            curr_line.push(token.clone());

            let peek = match peekable_iter.peek() {
                Some(&tok) => tok,
                None => break,
            };
            println!("token: {token:?}");
            match (token, peek) {
                (Token::Const | Token::Let | Token::Var, Token::Identifier(ref ident)) => {
                    peekable_iter.next().unwrap();
                    parser.inc_curr_index();
                    let next_token = peekable_iter.next().unwrap();
                    parser.inc_curr_index();
                    let next_peek = *peekable_iter.peek().unwrap_or(&&Token::Semicolon);

                    println!("next_token: {next_token:?}");

                    match (next_token, next_peek) {
                        (Token::Semicolon, _) => {
                            println!(" (Token::Semicolon, _)");

                            let new_ident = Identifier {
                                name: ident.to_string(),
                                value: Value::Undefined,
                                kind: IdentKind::Variable,
                                declared_at: parser.get_curr_pos(),
                                used_at: None,
                                mutated_at: None,
                            };
                            parser.push_new_ident(new_ident);

                            curr_line.kind = StatementKind::Declaration;
                            parser.lines.push(curr_line);

                            curr_line = parser.new_line();
                        }
                        (Token::Equals, Token::Number(num)) => {
                            println!(" (Token::Equals, _)");

                            peekable_iter.next().unwrap();
                            // let next_token = peekable_iter.next().unwrap();
                            // let next_peek = *peekable_iter.peek().unwrap_or(&&Token::Semicolon);
                            // parser.inc_curr_index();

                            println!("next_token: {next_token:?}");
                            println!("next_peek: {next_peek:?}");

                            let new_ident = Identifier {
                                name: ident.to_string(),
                                value: Value::Constant(num.to_string()),
                                kind: IdentKind::Variable,
                                declared_at: parser.get_curr_pos(),
                                used_at: None,
                                mutated_at: None,
                            };
                            parser.push_new_ident(new_ident);
                        }
                        _ => (),
                    }
                    //could be a variable declaration or closure declaration
                }
                (Token::Semicolon, _) => {
                    curr_line = parser.new_line();
                }
                _ => {}
            }
            // if *token == Token::Semicolon || *token == Token::LSquirly || *token == Token::RSquirly {
            //     let line_no = line.line_no;

            //     lines.push(line);
            //     line = Line::new();
            //     line.set_line_no(line_no + 1);
            // }
        }
        println!("ident list: {idents:?}", idents = parser.sym_tab);
        parser
    }

    fn inc_curr_index(&mut self) {
        self.curr_index += 1;
    }
    fn inc_curr_line_no(&mut self) {
        self.curr_line_no += 1;
    }
    fn get_curr_pos(&self) -> TokenPos {
        TokenPos(self.curr_line_no, self.curr_index)
    }
    fn push_new_ident(&mut self, ident: Identifier) {
        self.sym_tab.push(ident);
    }
    fn new_line(&mut self) -> Statement {
        self.inc_curr_line_no();
        Statement::new_line(self.curr_line_no)
    }
}
