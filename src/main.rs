mod ast;

use ast::{evaluator::ASTEvaluator, lexer::Lexer};

use crate::ast::{parser::Parser, Ast};

fn main() {
    let input = "7 - (30 + 7) * 8 / 2";

    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    for token in &tokens {
        println!("{:?}", token);
    }

    let mut ast = Ast::new();
    let mut parser = Parser::new(tokens);

    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }
    ast.visualize();

    let mut evaluator = ASTEvaluator::new();
    ast.visit(&mut evaluator);

    println!("Result: {:?}", evaluator.last_value.unwrap())
}
