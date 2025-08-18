use std::collections::HashMap;
use crate::parser::{Expr, Stmt, Literal};
use crate::lexer::Token;
use crate::error::{Result, SunscreenError};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Function(Function),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Function(func) => write!(f, "<fn {}>", func.name),
        }
    }
}

pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_enclosing(enclosing: Environment) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<Value> {
        if let Some(value) = self.values.get(name) {
            Ok(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.get(name)
        } else {
            Err(SunscreenError::RuntimeError(
                format!("Undefined variable '{}'.", name)
            ))
        }
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<()> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign(name, value)
        } else {
            Err(SunscreenError::RuntimeError(
                format!("Undefined variable '{}'.", name)
            ))
        }
    }
}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<()> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: Stmt) -> Result<()> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
                Ok(())
            }
            Stmt::Print(expr) => {
                let value = self.evaluate(expr)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::Var { name, initializer } => {
                let value = if let Some(init) = initializer {
                    self.evaluate(init)?
                } else {
                    Value::Nil
                };
                self.environment.define(name, value);
                Ok(())
            }
            Stmt::Block(statements) => {
                let new_env = Environment::new_with_enclosing(
                    std::mem::replace(&mut self.environment, Environment::new())
                );
                self.execute_block(statements, new_env)
            }
            Stmt::If { condition, then_branch, else_branch } => {
                let condition_value = self.evaluate(condition)?;
                if self.is_truthy(&condition_value) {
                    self.execute(*then_branch)?;
                } else if let Some(else_stmt) = else_branch {
                    self.execute(*else_stmt)?;
                }
                Ok(())
            }
            Stmt::While { condition, body } => {
                loop {
                    let condition_value = self.evaluate(condition.clone())?;
                    if !self.is_truthy(&condition_value) {
                        break;
                    }
                    self.execute(*body.clone())?;
                }
                Ok(())
            }
            Stmt::Function { name, params, body } => {
                let function = Function { name: name.clone(), params, body };
                self.environment.define(name, Value::Function(function));
                Ok(())
            }
            Stmt::Return(value) => {
                let return_value = if let Some(expr) = value {
                    self.evaluate(expr)?
                } else {
                    Value::Nil
                };
                return Err(SunscreenError::RuntimeError(
                    format!("Return value: {:?}", return_value)
                ));
            }
        }
    }

    fn execute_block(&mut self, statements: Vec<Stmt>, environment: Environment) -> Result<()> {
        let previous = std::mem::replace(&mut self.environment, environment);
        
        for statement in statements {
            self.execute(statement)?;
        }
        
        self.environment = previous;
        Ok(())
    }

    fn evaluate(&mut self, expr: Expr) -> Result<Value> {
        match expr {
            Expr::Literal(literal) => Ok(self.literal_to_value(literal)),
            Expr::Variable(name) => self.environment.get(&name),
            Expr::Binary { left, operator, right } => {
                let left_val = self.evaluate(*left)?;
                let right_val = self.evaluate(*right)?;
                self.binary_operation(left_val, operator, right_val)
            }
            Expr::Unary { operator, right } => {
                let right_val = self.evaluate(*right)?;
                self.unary_operation(operator, right_val)
            }
            Expr::Call { callee, arguments } => {
                let callee_val = self.evaluate(*callee)?;
                let mut arg_values = Vec::new();
                
                for arg in arguments {
                    arg_values.push(self.evaluate(arg)?);
                }
                
                self.call_function(callee_val, arg_values)
            }
            Expr::Assign { name, value } => {
                let value = self.evaluate(*value)?;
                self.environment.assign(&name, value.clone())?;
                Ok(value)
            }
        }
    }

    fn literal_to_value(&self, literal: Literal) -> Value {
        match literal {
            Literal::Number(n) => Value::Number(n),
            Literal::String(s) => Value::String(s),
            Literal::Boolean(b) => Value::Boolean(b),
            Literal::Nil => Value::Nil,
        }
    }

    fn binary_operation(&self, left: Value, operator: Token, right: Value) -> Result<Value> {
        match (left, operator, right) {
            // Arithmetic operations
            (Value::Number(l), Token::Plus, Value::Number(r)) => Ok(Value::Number(l + r)),
            (Value::Number(l), Token::Minus, Value::Number(r)) => Ok(Value::Number(l - r)),
            (Value::Number(l), Token::Star, Value::Number(r)) => Ok(Value::Number(l * r)),
            (Value::Number(l), Token::Slash, Value::Number(r)) => {
                if r == 0.0 {
                    Err(SunscreenError::RuntimeError("Division by zero.".to_string()))
                } else {
                    Ok(Value::Number(l / r))
                }
            }
            
            // String concatenation
            (Value::String(l), Token::Plus, Value::String(r)) => Ok(Value::String(l + &r)),
            (Value::String(l), Token::Plus, Value::Number(r)) => Ok(Value::String(l + &r.to_string())),
            (Value::Number(l), Token::Plus, Value::String(r)) => Ok(Value::String(l.to_string() + &r)),
            
            // Comparison operations
            (Value::Number(l), Token::Greater, Value::Number(r)) => Ok(Value::Boolean(l > r)),
            (Value::Number(l), Token::GreaterEqual, Value::Number(r)) => Ok(Value::Boolean(l >= r)),
            (Value::Number(l), Token::Less, Value::Number(r)) => Ok(Value::Boolean(l < r)),
            (Value::Number(l), Token::LessEqual, Value::Number(r)) => Ok(Value::Boolean(l <= r)),
            
            // Equality operations
            (l, Token::EqualEqual, r) => Ok(Value::Boolean(self.values_equal(&l, &r))),
            (l, Token::BangEqual, r) => Ok(Value::Boolean(!self.values_equal(&l, &r))),
            
            _ => Err(SunscreenError::RuntimeError(
                format!("Invalid binary operation")
            )),
        }
    }

    fn unary_operation(&self, operator: Token, right: Value) -> Result<Value> {
        match (operator, right) {
            (Token::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
            (Token::Bang, value) => Ok(Value::Boolean(!self.is_truthy(&value))),
            _ => Err(SunscreenError::RuntimeError(
                format!("Invalid unary operation")
            )),
        }
    }

    fn call_function(&mut self, callee: Value, arguments: Vec<Value>) -> Result<Value> {
        match callee {
            Value::Function(function) => {
                if arguments.len() != function.params.len() {
                    return Err(SunscreenError::RuntimeError(
                        format!("Expected {} arguments but got {}.", 
                                function.params.len(), arguments.len())
                    ));
                }
                
                let mut environment = Environment::new_with_enclosing(
                    std::mem::replace(&mut self.environment, Environment::new())
                );
                
                for (param, arg) in function.params.iter().zip(arguments.iter()) {
                    environment.define(param.clone(), arg.clone());
                }
                
                // Execute function body
                let previous = std::mem::replace(&mut self.environment, environment);
                
                for stmt in function.body {
                    match self.execute(stmt) {
                        Ok(()) => continue,
                        Err(SunscreenError::RuntimeError(msg)) if msg.starts_with("Return value:") => {
                            // Extract return value from error message (temporary hack)
                            let _return_str = msg.strip_prefix("Return value: ").unwrap_or("nil");
                            // For now, just return nil
                            self.environment = previous;
                            return Ok(Value::Nil);
                        }
                        Err(e) => {
                            self.environment = previous;
                            return Err(e);
                        }
                    }
                }
                
                self.environment = previous;
                Ok(Value::Nil)
            }
            _ => Err(SunscreenError::RuntimeError(
                format!("Can only call functions and classes.")
            )),
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Boolean(b) => *b,
            Value::Nil => false,
            _ => true,
        }
    }

    fn values_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }
} 