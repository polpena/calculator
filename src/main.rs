use std::io::{self, BufRead};
use pest::Parser;

mod ast;
mod parser;


use crate::parser::parse_expr;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar="calc.pest"]
pub struct CalculatorParser;

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    for line in handle.lines() {
        let line = line?;
        match CalculatorParser::parse(Rule::equation, &line) {
            Ok(mut pairs) => {
                let expr = parse_expr(pairs.next().unwrap().into_inner());
                println!("Result: {}", expr.evaluate());
            }
            Err(e) => {
                eprintln!("Parse failed: {:?}", e);
            }
        }
    }
    Ok(())
}

