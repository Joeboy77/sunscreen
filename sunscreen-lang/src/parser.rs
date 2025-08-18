use crate::lexer::Token;
use crate::error::Result;

#[derive(Debug, Clone)]
pub enum Expr {
    // Literals
    Literal(Literal),
    Variable(String),
    
    // Binary operations
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    
    // Unary operations
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    
    // Function calls
    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
    
    // Assignment
    Assign {
        name: String,
        value: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var {
        name: String,
        initializer: Option<Expr>,
    },
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Return(Option<Expr>),
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt> {
        if self.match_token(&Token::Fn) {
            return self.function_declaration();
        }
        
        if self.match_token(&Token::Let) {
            return self.var_declaration();
        }
        
        self.statement()
    }

    fn function_declaration(&mut self) -> Result<Stmt> {
        let name = self.consume_identifier("Expect function name.")?;
        
        self.consume(&Token::LeftParen, "Expect '(' after function name.")?;
        
        let mut parameters = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                parameters.push(self.consume_identifier("Expect parameter name.")?);
                
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        
        self.consume(&Token::RightParen, "Expect ')' after parameters.")?;
        self.consume(&Token::LeftBrace, "Expect '{' before function body.")?;
        
        let body = self.block()?;
        
        Ok(Stmt::Function { name, params: parameters, body })
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = self.consume_identifier("Expect variable name.")?;
        
        let initializer = if self.match_token(&Token::Equal) {
            Some(self.expression()?)
        } else {
            None
        };
        
        self.consume(&Token::Semicolon, "Expect ';' after variable declaration.")?;
        
        Ok(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self.match_token(&Token::Print) {
            return self.print_statement();
        }
        
        if self.match_token(&Token::LeftBrace) {
            return Ok(Stmt::Block(self.block()?));
        }
        
        if self.match_token(&Token::If) {
            return self.if_statement();
        }
        
        if self.match_token(&Token::While) {
            return self.while_statement();
        }
        
        if self.match_token(&Token::Return) {
            return self.return_statement();
        }
        
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let value = self.expression()?;
        self.consume(&Token::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(value))
    }

    fn if_statement(&mut self) -> Result<Stmt> {
        self.consume(&Token::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(&Token::RightParen, "Expect ')' after if condition.")?;
        
        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_token(&Token::Else) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        
        Ok(Stmt::If { condition, then_branch, else_branch })
    }

    fn while_statement(&mut self) -> Result<Stmt> {
        self.consume(&Token::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(&Token::RightParen, "Expect ')' after condition.")?;
        
        let body = Box::new(self.statement()?);
        
        Ok(Stmt::While { condition, body })
    }

    fn return_statement(&mut self) -> Result<Stmt> {
        let value = if !self.check(&Token::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        
        self.consume(&Token::Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::Return(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(&Token::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(expr))
    }

    fn block(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();
        
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        
        self.consume(&Token::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        let expr = self.or()?;
        
        if self.match_token(&Token::Equal) {
            let value = self.assignment()?;
            
            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign { name, value: Box::new(value) });
            }
            
            return Err(crate::error::SunscreenError::ParserError(
                "Invalid assignment target.".to_string()
            ));
        }
        
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr> {
        let mut expr = self.and()?;
        
        while self.match_token(&Token::BangEqual) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr> {
        let mut expr = self.equality()?;
        
        while self.match_token(&Token::EqualEqual) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;
        
        while self.match_any(&[&Token::BangEqual, &Token::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;
        
        while self.match_any(&[&Token::Greater, &Token::GreaterEqual, &Token::Less, &Token::LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;
        
        while self.match_any(&[&Token::Minus, &Token::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;
        
        while self.match_any(&[&Token::Slash, &Token::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.match_any(&[&Token::Bang, &Token::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        
        self.call()
    }

    fn call(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;
        
        loop {
            if self.match_token(&Token::LeftParen) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr> {
        let mut arguments = Vec::new();
        
        if !self.check(&Token::RightParen) {
            loop {
                arguments.push(self.expression()?);
                
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        
        self.consume(&Token::RightParen, "Expect ')' after arguments.")?;
        
        Ok(Expr::Call {
            callee: Box::new(callee),
            arguments,
        })
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.match_token(&Token::False) {
            return Ok(Expr::Literal(Literal::Boolean(false)));
        }
        if self.match_token(&Token::True) {
            return Ok(Expr::Literal(Literal::Boolean(true)));
        }
        if self.match_token(&Token::Nil) {
            return Ok(Expr::Literal(Literal::Nil));
        }
        
        if let Some(token) = self.advance_if(&Token::Number(0.0)) {
            if let Token::Number(n) = token {
                return Ok(Expr::Literal(Literal::Number(n)));
            }
        }
        
        if let Some(token) = self.advance_if(&Token::String("".to_string())) {
            if let Token::String(s) = token {
                return Ok(Expr::Literal(Literal::String(s)));
            }
        }
        
        if let Some(token) = self.advance_if(&Token::Identifier("".to_string())) {
            if let Token::Identifier(n) = token {
                return Ok(Expr::Variable(n));
            }
        }
        
        if self.match_token(&Token::LeftParen) {
            let expr = self.expression()?;
            self.consume(&Token::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Unary {
                operator: Token::LeftParen,
                right: Box::new(expr),
            });
        }
        
        Err(crate::error::SunscreenError::ParserError(
            "Expect expression.".to_string()
        ))
    }

    // Helper methods
    fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_any(&mut self, tokens: &[&Token]) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(self.peek()) == std::mem::discriminant(token)
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn advance_if(&mut self, token: &Token) -> Option<Token> {
        if self.check(token) {
            Some(self.advance())
        } else {
            None
        }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn consume(&mut self, token: &Token, message: &str) -> Result<()> {
        if self.check(token) {
            self.advance();
            Ok(())
        } else {
            Err(crate::error::SunscreenError::ParserError(message.to_string()))
        }
    }

    fn consume_identifier(&mut self, message: &str) -> Result<String> {
        if let Token::Identifier(name) = self.peek() {
            let name = name.clone();
            self.advance();
            Ok(name)
        } else {
            Err(crate::error::SunscreenError::ParserError(message.to_string()))
        }
    }
} 