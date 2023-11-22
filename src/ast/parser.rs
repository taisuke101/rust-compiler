use super::{
    lexer::{TextSpan, Token, TokenKind},
    ASTBinaryOperator, ASTBinaryOperatorKind, ASTExpression, ASTStatement,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens
                .iter()
                .filter(|token: &&Token| token.kind != TokenKind::Whitespace)
                .map(|token| token.clone())
                .collect(),
            current: 0,
        }
    }

    pub fn next_statement(&mut self) -> Option<ASTStatement> {
        return self.parse_statement();
    }

    fn parse_statement(&mut self) -> Option<ASTStatement> {
        let token = self.current()?;
        if token.kind == TokenKind::Eof {
            return None;
        }
        let expression = self.parse_expression()?;

        return Some(ASTStatement::expression(expression));
    }

    fn parse_expression(&mut self) -> Option<ASTExpression> {
        self.parse_binary_expression(0)
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> Option<ASTExpression> {
        let mut left = self.parse_primary_expression()?;

        while let Some(operator) = self.parse_binary_operator() {
            self.consume();
            let operator_precedence = operator.precedence();
            if operator_precedence < precedence {
                break;
            }
            let right = self.parse_binary_expression(operator_precedence)?;
            left = ASTExpression::binary(operator, left, right);
        }

        return Some(left);
    }

    fn parse_binary_operator(&mut self) -> Option<ASTBinaryOperator> {
        self.current().and_then(|token| {
            let kind = match token.kind {
                TokenKind::Plus => Some(ASTBinaryOperatorKind::Plus),
                TokenKind::Minus => Some(ASTBinaryOperatorKind::Minus),
                TokenKind::Asterisk => Some(ASTBinaryOperatorKind::Multiply),
                TokenKind::Slash => Some(ASTBinaryOperatorKind::Divide),
                _ => None,
            };
            kind.map(|kind| ASTBinaryOperator::new(kind, token.clone()))
        })
    }

    fn parse_primary_expression(&mut self) -> Option<ASTExpression> {
        let token = self.consume()?;
        match token.kind {
            TokenKind::Number(number) => Some(ASTExpression::number(number)),
            TokenKind::LeftParen => {
                let expression = self.parse_expression()?;

                let token = self.consume()?;

                if token.kind != TokenKind::RightParen {
                    panic!("Expected Right Paren");
                }
                Some(ASTExpression::parenthesized(expression))
            }
            _ => None,
        }
    }

    fn peek(&self, offset: isize) -> Option<&Token> {
        self.tokens.get((self.current as isize + offset) as usize)
    }

    fn current(&self) -> Option<&Token> {
        self.peek(0)
    }

    fn consume(&mut self) -> Option<&Token> {
        self.current += 1;
        let token = self.peek(-1)?;
        return Some(token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_binary_operator() {
        let mut parser = Parser::new(vec![
            Token::new(
                TokenKind::Plus,
                TextSpan {
                    start: 0,
                    end: 1,
                    literal: "+".to_string(),
                },
            ),
            Token::new(
                TokenKind::Minus,
                TextSpan {
                    start: 1,
                    end: 2,
                    literal: "-".to_string(),
                },
            ),
            Token::new(
                TokenKind::Asterisk,
                TextSpan {
                    start: 1,
                    end: 3,
                    literal: "*".to_string(),
                },
            ),
            Token::new(
                TokenKind::Slash,
                TextSpan {
                    start: 2,
                    end: 3,
                    literal: "/".to_string(),
                },
            ),
            Token::new(
                TokenKind::Number(123),
                TextSpan {
                    start: 2,
                    end: 3,
                    literal: "123".to_string(),
                },
            ),
        ]);

        let expected_kinds = [
            ASTBinaryOperatorKind::Plus,
            ASTBinaryOperatorKind::Minus,
            ASTBinaryOperatorKind::Multiply,
            ASTBinaryOperatorKind::Divide,
        ];

        for expected_kind in expected_kinds.iter() {
            assert_eq!(parser.parse_binary_operator().unwrap().kind, *expected_kind);
            parser.current += 1;
        }

        assert!(parser.parse_binary_operator().is_none());
    }
}
