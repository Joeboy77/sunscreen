use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Number(f64),
    String(String),
    Identifier(String),
    
    // Keywords
    Let,
    If,
    Else,
    While,
    For,
    Fn,
    Return,
    True,
    False,
    Print,
    Nil,
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    Dot,
    
    // Special
    Eof,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }
        
        tokens.push(Token::Eof);
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>, String> {
        self.skip_whitespace();
        
        if let Some(&ch) = self.source.peek() {
            match ch {
                // Single character tokens
                '(' => {
                    self.advance();
                    Ok(Some(Token::LeftParen))
                }
                ')' => {
                    self.advance();
                    Ok(Some(Token::RightParen))
                }
                '{' => {
                    self.advance();
                    Ok(Some(Token::LeftBrace))
                }
                '}' => {
                    self.advance();
                    Ok(Some(Token::RightBrace))
                }
                ';' => {
                    self.advance();
                    Ok(Some(Token::Semicolon))
                }
                ',' => {
                    self.advance();
                    Ok(Some(Token::Comma))
                }
                '.' => {
                    self.advance();
                    Ok(Some(Token::Dot))
                }
                '+' => {
                    self.advance();
                    Ok(Some(Token::Plus))
                }
                '-' => {
                    self.advance();
                    Ok(Some(Token::Minus))
                }
                '*' => {
                    self.advance();
                    Ok(Some(Token::Star))
                }
                '/' => {
                    self.advance();
                    if self.match_char('/') {
                        // Comment - skip to end of line
                        while self.peek() != '\n' && self.peek() != '\0' {
                            self.advance();
                        }
                        self.next_token()
                    } else {
                        Ok(Some(Token::Slash))
                    }
                }
                
                // Two character tokens
                '=' => {
                    self.advance();
                    if self.match_char('=') {
                        Ok(Some(Token::EqualEqual))
                    } else {
                        Ok(Some(Token::Equal))
                    }
                }
                '!' => {
                    self.advance();
                    if self.match_char('=') {
                        Ok(Some(Token::BangEqual))
                    } else {
                        Ok(Some(Token::Bang))
                    }
                }
                '<' => {
                    self.advance();
                    if self.match_char('=') {
                        Ok(Some(Token::LessEqual))
                    } else {
                        Ok(Some(Token::Less))
                    }
                }
                '>' => {
                    self.advance();
                    if self.match_char('=') {
                        Ok(Some(Token::GreaterEqual))
                    } else {
                        Ok(Some(Token::Greater))
                    }
                }
                
                // Strings
                '"' => {
                    self.advance();
                    self.string()
                }
                
                // Numbers and identifiers
                '0'..='9' => self.number(),
                'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
                
                _ => {
                    let ch = self.advance();
                    Err(format!("Unexpected character '{}' at line {}:{}", ch, self.line, self.column))
                }
            }
        } else {
            Ok(None)
        }
    }

    fn string(&mut self) -> Result<Option<Token>, String> {
        let mut value = String::new();
        
        while self.peek() != '"' && self.peek() != '\0' {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            value.push(self.advance());
        }
        
        if self.peek() == '\0' {
            return Err("Unterminated string".to_string());
        }
        
        // Consume the closing quote
        self.advance();
        
        Ok(Some(Token::String(value)))
    }

    fn number(&mut self) -> Result<Option<Token>, String> {
        let mut value = String::new();
        
        // Integer part
        while self.peek().is_ascii_digit() {
            value.push(self.advance());
        }
        
        // Fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            value.push(self.advance()); // consume the '.'
            
            while self.peek().is_ascii_digit() {
                value.push(self.advance());
            }
        }
        
        match value.parse::<f64>() {
            Ok(num) => Ok(Some(Token::Number(num))),
            Err(_) => Err(format!("Invalid number: {}", value)),
        }
    }

    fn identifier(&mut self) -> Result<Option<Token>, String> {
        let mut value = String::new();
        
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            value.push(self.advance());
        }
        
        // Check for keywords
        let token = match value.as_str() {
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "fn" => Token::Fn,
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            "print" => Token::Print,
            _ => Token::Identifier(value),
        };
        
        Ok(Some(token))
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.source.peek() {
            match ch {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.column = 1;
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn advance(&mut self) -> char {
        self.column += 1;
        self.source.next().unwrap_or('\0')
    }

    fn peek(&mut self) -> char {
        self.source.peek().copied().unwrap_or('\0')
    }

    fn peek_next(&mut self) -> char {
        let mut iter = self.source.clone();
        iter.next();
        iter.peek().copied().unwrap_or('\0')
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == expected {
            self.advance();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("123 + 456");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens, vec![
            Token::Number(123.0),
            Token::Plus,
            Token::Number(456.0),
            Token::Eof,
        ]);
    }

    #[test]
    fn test_strings() {
        let mut lexer = Lexer::new(r#""hello world""#);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens, vec![
            Token::String("hello world".to_string()),
            Token::Eof,
        ]);
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("let x = 42");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens, vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Equal,
            Token::Number(42.0),
            Token::Eof,
        ]);
    }
} 