# C Compiler Driver

A simple compiler driver written in Rust that automates the compilation process for C source files. This project is part of my compiler study journey following a book on compiler construction.

## What is a Compiler Driver?

A compiler driver is a program that orchestrates the entire compilation process by invoking different tools in the correct sequence. This driver handles the three main stages of C compilation:

1. **Preprocessing** - Expands macros, includes header files, and removes comments
2. **Compilation** - Converts preprocessed C code to assembly language
3. **Assembly** - Converts assembly code to object files

## Features

- Automated compilation pipeline for C source files
- Clean intermediate file management (automatically removes `.i` and `.s` files)
- Clear status messages for each compilation stage
- Simple command-line interface

## Prerequisites

- Rust (for building the driver)
- GCC (for the actual compilation stages)

## Building

```bash
cargo build --release
```

## Usage

### Basic Usage

```bash
./target/release/compiler_driver <source_file.c>
```

### Example

1. Create a simple C file:
```c
// hello.c
#include <stdio.h>

int main(void) {
    printf("Hello, World!\n");
    return 0;
}
```

2. Compile it using the driver:
```bash
./target/release/compiler_driver hello.c
```

3. The driver will output:
```
Preprocessing succeeded!
Compilation to assembly succeeded!
Assembly to object file succeeded!
```

4. You'll find `hello.o` (object file) in the same directory.

### Complete Example with Linking

To create an executable, you still need to link the object file:

```bash
# Compile with the driver
./target/release/compiler_driver hello.c

# Link to create executable
gcc hello.o -o hello

# Run the program
./hello
```

## How It Works

The compiler driver performs these steps:

1. **Preprocessing** (`gcc -E -P`)
   - Input: `source.c`
   - Output: `source.i` (preprocessed file)

2. **Compilation** (`gcc -S`)
   - Input: `source.i`
   - Output: `source.s` (assembly file)
   - Cleans up: `source.i`

3. **Assembly** (`gcc -c`)
   - Input: `source.s`
   - Output: `source.o` (object file)
   - Cleans up: `source.s`

## Project Structure

```
compiler_driver/
├── src/
│   └── main.rs          # Main driver implementation
├── Cargo.toml           # Rust project configuration
├── README.md            # This file
└── tests/               # Integration tests
```

## Example C Programs

The repository includes sample C programs for testing:

- `return_2.c` - Simple program that returns exit code 2

## Testing

The project includes comprehensive integration tests that verify the compiler driver's functionality:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Test Coverage

The integration tests cover:

-  Successful compilation of simple C files
-  Compilation of C files with includes
-  Handling of invalid C syntax (compilation failures)


### Test Files

Test files are located in `tests/test_files/`:
- `simple_return.c` - Basic C program
- `with_includes.c` - C program with standard library includes
- `invalid_syntax.c` - C program with syntax errors (for failure testing)

## Contributing

This is a learning project, but feel free to open issues or submit pull requests if you find bugs or have suggestions for improvements.

## Learning Resources

This compiler driver is part of my study of compiler construction. It demonstrates the basic pipeline that more sophisticated compilers follow, albeit with much more complex intermediate representations and optimizations.

## License

This project is for educational purposes.