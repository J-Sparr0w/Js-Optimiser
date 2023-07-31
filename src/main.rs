use js_optimizer::lexer::{Lexer, Token};
use js_optimizer::parser::{Parser, Statement};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

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

    let mut output_file = File::create("output.js").unwrap();

    let mut out_buf = String::new();

    // for tok in lexer.tokens.iter() {
    //     out_buf.push_str(&tok.to_str());
    // }

    // match output_file.write_all(&out_buf.as_bytes()) {
    //     Ok(_) => println!("Successfully written to file output.js"),
    //     Err(err) => println!("ERROR: couldn't write to file: {err}"),
    // }
    // let mut lines: Vec<Line> = Vec::new();
    let parser = Parser::parse_from_lexer(&lexer);

    // for line in lines.iter().peekable() {
    //     println!("{line:?}");
    //     match *line.tokens {
    //         [Token::Const | Token::Let | Token::Var, Token::Identifier(ref id), Token::Equals, Token::Number(ref num), Token::Semicolon] =>
    //         {
    //             println!(
    //                 "[const,Identifier,=,number,semicolon]: {toks:?}",
    //                 toks = line.tokens
    //             )
    //         }
    //         [Token::Const | Token::Let | Token::Var, ..] => {
    //             println!("line: [const,..]: {toks:?}", toks = line.tokens)
    //         }
    //         _ => {}
    //     }
    // }
}
