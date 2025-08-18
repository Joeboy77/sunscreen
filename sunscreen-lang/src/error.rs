use std::fmt;

#[derive(Debug)]
pub enum SunscreenError {
    LexerError(String),
    ParserError(String),
    RuntimeError(String),
    IoError(std::io::Error),
}

impl fmt::Display for SunscreenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SunscreenError::LexerError(msg) => write!(f, "Lexer error: {}", msg),
            SunscreenError::ParserError(msg) => write!(f, "Parser error: {}", msg),
            SunscreenError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
            SunscreenError::IoError(err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl std::error::Error for SunscreenError {}

impl From<std::io::Error> for SunscreenError {
    fn from(err: std::io::Error) -> Self {
        SunscreenError::IoError(err)
    }
}

impl From<String> for SunscreenError {
    fn from(err: String) -> Self {
        SunscreenError::RuntimeError(err)
    }
}

impl From<&str> for SunscreenError {
    fn from(err: &str) -> Self {
        SunscreenError::RuntimeError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, SunscreenError>;

pub struct ErrorReporter {
    had_error: bool,
}

impl ErrorReporter {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
        self.had_error = true;
    }

    pub fn error_at_token(&mut self, token: &crate::lexer::Token, message: &str) {
        if let crate::lexer::Token::Eof = token {
            self.report(0, " at end", message);
        } else {
            self.report(0, &format!(" at '{}'", self.token_to_string(token)), message);
        }
        self.had_error = true;
    }

    fn report(&self, line: usize, location: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, location, message);
    }

    fn token_to_string(&self, token: &crate::lexer::Token) -> String {
        match token {
            crate::lexer::Token::Number(n) => n.to_string(),
            crate::lexer::Token::String(s) => s.clone(),
            crate::lexer::Token::Identifier(s) => s.clone(),
            crate::lexer::Token::Let => "let".to_string(),
            crate::lexer::Token::If => "if".to_string(),
            crate::lexer::Token::Else => "else".to_string(),
            crate::lexer::Token::While => "while".to_string(),
            crate::lexer::Token::For => "for".to_string(),
            crate::lexer::Token::Fn => "fn".to_string(),
            crate::lexer::Token::Return => "return".to_string(),
            crate::lexer::Token::True => "true".to_string(),
            crate::lexer::Token::False => "false".to_string(),
            crate::lexer::Token::Print => "print".to_string(),
            crate::lexer::Token::Nil => "nil".to_string(),
            crate::lexer::Token::Plus => "+".to_string(),
            crate::lexer::Token::Minus => "-".to_string(),
            crate::lexer::Token::Star => "*".to_string(),
            crate::lexer::Token::Slash => "/".to_string(),
            crate::lexer::Token::Equal => "=".to_string(),
            crate::lexer::Token::EqualEqual => "==".to_string(),
            crate::lexer::Token::Bang => "!".to_string(),
            crate::lexer::Token::BangEqual => "!=".to_string(),
            crate::lexer::Token::Less => "<".to_string(),
            crate::lexer::Token::LessEqual => "<=".to_string(),
            crate::lexer::Token::Greater => ">".to_string(),
            crate::lexer::Token::GreaterEqual => ">=".to_string(),
            crate::lexer::Token::LeftParen => "(".to_string(),
            crate::lexer::Token::RightParen => ")".to_string(),
            crate::lexer::Token::LeftBrace => "{".to_string(),
            crate::lexer::Token::RightBrace => "}".to_string(),
            crate::lexer::Token::Semicolon => ";".to_string(),
            crate::lexer::Token::Comma => ",".to_string(),
            crate::lexer::Token::Dot => ".".to_string(),
            crate::lexer::Token::Eof => "end of file".to_string(),
        }
    }

    pub fn had_error(&self) -> bool {
        self.had_error
    }

    pub fn reset(&mut self) {
        self.had_error = false;
    }
} 