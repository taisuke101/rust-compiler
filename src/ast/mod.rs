use self::lexer::Token;

pub mod evaluator;
pub mod lexer;
pub mod parser;

pub struct Ast {
    pub statements: Vec<ASTStatement>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: ASTStatement) {
        self.statements.push(statement);
    }

    pub fn visit(&self, visitor: &mut dyn ASTVisitor) {
        for statement in &self.statements {
            visitor.visit_statement(statement);
        }
    }

    pub fn visualize(&self) {
        let mut printer = ASTPrinter { indent: 0 };
        self.visit(&mut printer);
    }
}

pub trait ASTVisitor {
    fn do_visit_statement(&mut self, statement: &ASTStatement) {
        match &statement.kind {
            ASTStatementKind::Expression(expression) => self.visit_expression(expression),
        }
    }
    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.do_visit_statement(statement)
    }
    fn do_visit_expression(&mut self, expression: &ASTExpression) {
        match &expression.kind {
            ASTExpressionKind::Number(number) => self.visit_number(number),
            ASTExpressionKind::Binary(expression) => self.visit_binary_expression(expression),
            ASTExpressionKind::Parenthesized(expression) => {
                self.visit_parenthesized_expression(expression)
            }
        }
    }
    fn visit_expression(&mut self, expression: &ASTExpression) {
        self.do_visit_expression(expression)
    }
    fn visit_number(&mut self, number: &ASTNumberExpression);

    fn visit_binary_expression(&mut self, expression: &ASTBinaryExpression) {
        self.visit_expression(&expression.left);
        self.visit_expression(&expression.right);
    }

    fn visit_parenthesized_expression(
        &mut self,
        parenthesized_expression: &ParenthesizedExpression,
    ) {
        self.visit_expression(&parenthesized_expression.expression)
    }
}

const LEVEL_INDENT: usize = 2;

impl ASTVisitor for ASTPrinter {
    fn visit_number(&mut self, number: &ASTNumberExpression) {
        self.print_with_indent(&format!("Number: {}", number.number));
    }

    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.print_with_indent("Statement:");
        self.indent += LEVEL_INDENT;
        ASTVisitor::do_visit_statement(self, statement);
        self.indent -= LEVEL_INDENT;
    }

    fn visit_expression(&mut self, expression: &ASTExpression) {
        self.print_with_indent("Expression:");
        self.indent += LEVEL_INDENT;
        ASTVisitor::do_visit_expression(self, expression);
        self.indent -= LEVEL_INDENT;
    }

    fn visit_binary_expression(&mut self, binary_expression: &ASTBinaryExpression) {
        self.print_with_indent("Binary Expression:");
        self.indent += LEVEL_INDENT;
        self.print_with_indent(&format!("Operator: {:?}", binary_expression.operator.kind));
        self.visit_expression(&binary_expression.left);
        self.visit_expression(&binary_expression.right);
        self.indent -= LEVEL_INDENT;
    }

    fn visit_parenthesized_expression(
        &mut self,
        parenthesized_expression: &ParenthesizedExpression,
    ) {
        self.print_with_indent("Parenthesized Expression:");
        self.indent += LEVEL_INDENT;
        self.visit_expression(&parenthesized_expression.expression);
        self.indent -= LEVEL_INDENT;
    }
}

pub struct ASTPrinter {
    indent: usize,
}

impl ASTPrinter {
    fn print_with_indent(&mut self, text: &str) {
        println!("{}{}", " ".repeat(self.indent), text)
    }
}

pub enum ASTStatementKind {
    Expression(ASTExpression),
}

pub struct ASTStatement {
    kind: ASTStatementKind,
}

impl ASTStatement {
    pub fn new(kind: ASTStatementKind) -> Self {
        ASTStatement { kind }
    }

    pub fn expression(expression: ASTExpression) -> Self {
        return ASTStatement::new(ASTStatementKind::Expression(expression));
    }
}

pub enum ASTExpressionKind {
    Number(ASTNumberExpression),
    Binary(ASTBinaryExpression),
    Parenthesized(ParenthesizedExpression),
}

#[derive(Debug, PartialEq)]
pub enum ASTBinaryOperatorKind {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub struct ASTBinaryOperator {
    kind: ASTBinaryOperatorKind,
    token: Token,
}

impl ASTBinaryOperator {
    pub fn new(kind: ASTBinaryOperatorKind, token: Token) -> Self {
        ASTBinaryOperator { kind, token }
    }

    pub fn precedence(&self) -> u8 {
        match self.kind {
            ASTBinaryOperatorKind::Plus => 1,
            ASTBinaryOperatorKind::Minus => 1,
            ASTBinaryOperatorKind::Multiply => 2,
            ASTBinaryOperatorKind::Divide => 2,
        }
    }
}
pub struct ASTBinaryExpression {
    left: Box<ASTExpression>,
    right: Box<ASTExpression>,
    operator: ASTBinaryOperator,
}

pub struct ASTNumberExpression {
    number: i64,
}

pub struct ParenthesizedExpression {
    expression: Box<ASTExpression>,
}

pub struct ASTExpression {
    kind: ASTExpressionKind,
}

impl ASTExpression {
    pub fn new(kind: ASTExpressionKind) -> Self {
        ASTExpression { kind }
    }

    pub fn number(number: i64) -> Self {
        return ASTExpression::new(ASTExpressionKind::Number(ASTNumberExpression { number }));
    }

    pub fn binary(operator: ASTBinaryOperator, left: ASTExpression, right: ASTExpression) -> Self {
        ASTExpression::new(ASTExpressionKind::Binary(ASTBinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }

    pub fn parenthesized(expression: ASTExpression) -> Self {
        ASTExpression::new(ASTExpressionKind::Parenthesized(ParenthesizedExpression {
            expression: Box::new(expression),
        }))
    }
}
