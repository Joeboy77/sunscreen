use clap::Parser;
use std::path::PathBuf;
use anyhow::Result;

mod lexer;
mod parser;
mod interpreter;
mod error;

#[derive(Parser)]
#[command(name = "sunscreen")]
#[command(about = "A fast, simple, and friendly programming language")]
struct Cli {
    /// The source file to execute
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,

    /// Run in interactive mode (REPL)
    #[arg(short, long)]
    interactive: bool,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        println!("🌞 Welcome to Sunscreen Language v0.1.0");
    }

    match (cli.file, cli.interactive) {
        (Some(file), _) => {
            // Execute a file
            if cli.verbose {
                println!("Executing file: {:?}", file);
            }
            // TODO: Implement file execution
            println!("File execution coming soon!");
        }
        (None, true) => {
            // Start REPL
            if cli.verbose {
                println!("Starting interactive mode...");
            }
            run_repl()?;
        }
        (None, false) => {
            // Default to REPL if no arguments
            if cli.verbose {
                println!("No file specified, starting interactive mode...");
            }
            run_repl()?;
        }
    }

    Ok(())
}

fn run_repl() -> Result<()> {
    use rustyline::DefaultEditor;
    use colored::*;

    let mut rl = DefaultEditor::new()?;
    let mut interpreter = interpreter::Interpreter::new();
    
    println!("{}", "🌞 Sunscreen Language REPL".green().bold());
    println!("{}", "Type 'exit' or 'quit' to exit".dimmed());
    println!();

    loop {
        let readline = rl.readline("sunscreen> ");
        match readline {
            Ok(line) => {
                if line.trim().is_empty() {
                    continue;
                }
                
                if line.trim() == "exit" || line.trim() == "quit" {
                    println!("{}", "Goodbye! 👋".green());
                    break;
                }

                // Parse and execute the input
                match parse_and_execute(&line, &mut interpreter) {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("{}", format!("Error: {}", e).red());
                    }
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        }
    }

    Ok(())
}

fn parse_and_execute(source: &str, interpreter: &mut interpreter::Interpreter) -> Result<()> {
    // Lexical analysis
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|e| crate::error::SunscreenError::LexerError(e))?;
    
    // Parsing
    let mut parser = parser::Parser::new(tokens);
    let statements = parser.parse()?;
    
    // Interpretation
    interpreter.interpret(statements)?;
    
    Ok(())
} 