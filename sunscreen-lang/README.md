# 🌞 Sunscreen Programming Language

A fast, simple, and friendly programming language built from scratch in Rust.

## Features

- **Simple Syntax**: Easy to learn and read
- **Dynamic Typing**: No type declarations needed
- **Basic Data Types**: Numbers, strings, booleans, and nil
- **Variables**: Declare and assign variables with `let`
- **Functions**: Define and call functions
- **Control Flow**: If/else statements and while loops
- **REPL**: Interactive mode for testing and learning

## Quick Start

### Building from Source

```bash
cd sunscreen-lang
cargo build --release
```

### Running the REPL

```bash
cargo run
# or
./target/release/sunscreen-lang
```

### Running a File

```bash
cargo run -- examples/hello.ss
```

## Language Syntax

### Variables

```rust
let x = 42;
let name = "Sunscreen";
let is_cool = true;
```

### Print Statements

```rust
print "Hello, World!";
print 42;
print x + y;
```

### Arithmetic

```rust
let a = 10 + 5;    // 15
let b = 10 * 5;    // 50
let c = 10 - 5;    // 5
let d = 10 / 5;    // 2
```

### Strings

```rust
let greeting = "Hello";
let name = "World";
print greeting + " " + name;  // "Hello World"
```

### Functions

```rust
fn add(a, b) {
    return a + b;
}

let result = add(5, 3);  // 8
```

### Conditionals

```rust
if (x > 10) {
    print "x is greater than 10";
} else {
    print "x is 10 or less";
}
```

### Loops

```rust
let i = 1;
while (i <= 5) {
    print i;
    i = i + 1;
}
```

## Examples

Check out the `examples/` directory for sample programs:

- `hello.ss` - Basic language features demonstration
- More examples coming soon!

## Development Status

###  Implemented
- Lexical analysis (tokenizer)
- Parser (AST generation)
- Interpreter (tree-walking)
- Basic data types (numbers, strings, booleans)
- Variables and assignment
- Print statements
- Functions (basic)
- If/else statements
- While loops
- REPL mode

###  In Progress
- Better error handling
- File I/O operations
- Arrays and objects
- More built-in functions

###  Planned
- Compiler (bytecode generation)
- Standard library
- Modules and imports
- Classes and objects
- Performance optimizations

## Contributing

This is a learning project! Feel free to experiment and contribute ideas.

## License

MIT License - feel free to use this code for learning and experimentation. 