#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    Bad,
    Eof,
    Whitespace,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TextSpan {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) literal: String,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self {
            start,
            end,
            literal,
        }
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            current_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos == self.input.len() {
            let eof_char = '\0';
            self.current_pos += 1;
            return Some(Token::new(
                TokenKind::Eof,
                TextSpan::new(0, 0, eof_char.to_string()),
            ));
        }

        let c = self.current_char();
        return c.map(|c| {
            let start = self.current_pos;
            let kind = if Self::is_number_start(&c) {
                let number: i64 = self.consume_number();
                TokenKind::Number(number)
            } else if Self::is_whitespace(&c) {
                self.consume();
                TokenKind::Whitespace
            } else {
                self.consume_punctuation()
            };

            let end = self.current_pos;
            let literal = self.input[start..end].to_string();
            let span = TextSpan::new(start, end, literal);
            Token::new(kind, span)
        });
    }

    fn consume_punctuation(&mut self) -> TokenKind {
        let c = self.consume().unwrap();

        match c {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            _ => TokenKind::Bad,
        }
    }

    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
    }

    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    fn consume(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            return None;
        }
        let c = self.current_char();
        self.current_pos += 1;

        c
    }

    fn consume_number(&mut self) -> i64 {
        let mut number: i64 = 0;
        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                self.consume().unwrap();
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            } else {
                break;
            }
        }

        // let number = self
        //     .input
        //     .chars()
        //     .take_while(|c| c.is_digit(10))
        //     .fold(0, |acc, c| {
        //         self.consume().unwrap();
        //         acc * 10 + c.to_digit(10).unwrap() as i64
        //     });

        number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_number_token() {
        let mut lexer = Lexer::new("123");
        let token = lexer.next_token().unwrap();
        match token.kind {
            TokenKind::Number(n) => assert_eq!(n, 123),
            _ => panic!("Expected number token"),
        }
    }

    #[test]
    fn test_lexer_eof_token() {
        let mut lexer = Lexer::new("");
        let token = lexer.next_token().unwrap();
        match token.kind {
            TokenKind::Eof => assert!(true),
            _ => panic!("Expected EOF token"),
        }
    }

    #[test]
    fn test_textspan_length() {
        let span = TextSpan::new(0, 5, "12345".to_string());
        assert_eq!(span.length(), 5);
    }

    #[test]
    fn test_token_creation() {
        let span = TextSpan::new(0, 3, "123".to_string());
        let token = Token::new(TokenKind::Number(123), span);
        match token.kind {
            TokenKind::Number(n) => assert_eq!(n, 123),
            _ => panic!("Expected number token"),
        }
    }

    #[test]
    fn test_consume_number() {
        let mut lexer = Lexer::new("12345");
        assert_eq!(lexer.consume_number(), 12345);

        let mut lexer = Lexer::new("67890");
        assert_eq!(lexer.consume_number(), 67890);

        let mut lexer = Lexer::new("123abc");
        assert_eq!(lexer.consume_number(), 123);

        let mut lexer = Lexer::new("");
        assert_eq!(lexer.consume_number(), 0);
    }
}
