# 🧮 Computor v1

A polynomial equation solver written in Rust as part of the 42 curriculum. This project solves polynomial equations up to degree 2 (quadratic equations) and displays solutions in both real and complex forms.

## 📋 Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [Project Structure](#project-structure)
- [Implementation Details](#implementation-details)
- [Learning Objectives](#learning-objectives)

## ✨ Features

- ✅ Parses polynomial equations in natural mathematical form
- ✅ Handles floating-point coefficients (e.g., `9.3 * X^2`)
- ✅ Solves linear equations (degree 1)
- ✅ Solves quadratic equations (degree 2) with real or complex solutions
- ✅ Detects identity equations (infinite solutions)
- ✅ Detects impossible equations (no solution)
- ✅ Displays reduced form of the equation
- ✅ Computes and displays the discriminant for quadratic equations
- ✅ Handles negative coefficients and complex number formatting
- 🎁 Bonus features available via `make bonus`

## 🚀 Installation

### Prerequisites

- Rust toolchain (rustc, cargo)
- Make

### Compilation
```bash
# Standard version
make

# With bonus features
make bonus

# Debug mode
make debug=1
```

## 💡 Usage
```bash
./computor ""
```

### Input Format

Equations must follow the format:
c * X^0 + b * X^1 + a * X^2 = ...

Where:
- Coefficients can be integers or floats
- Terms can appear in any order
- Signs (`+` or `-`) are supported
- Spaces are optional

## 📝 Examples

### Quadratic Equation (Two Real Solutions)
```bash
$ ./computor "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0"
Reduced form: 4 * X^0 + 4 * X^1 - 9.3 * X^2 = 0
Polynomial degree: 2
Discriminant is strictly positive, the two solutions are:
0.905239
-0.475131
```

### Linear Equation
```bash
$ ./computor "5 * X^0 + 4 * X^1 = 4 * X^0"
Reduced form: 1 * X^0 + 4 * X^1 = 0
Polynomial degree: 1
The solution is:
-0.25
```

### Degree > 2 (Unsolvable)
```bash
$ ./computor "8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0"
Reduced form: 5 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 0
Polynomial degree: 3
The polynomial degree is strictly greater than 2, I can't solve.
```

### Identity (Infinite Solutions)
```bash
$ ./computor "6 * X^0 = 6 * X^0"
Reduced form: 0 * X^0 = 0
Any real number is a solution.
```

### No Solution
```bash
$ ./computor "10 * X^0 = 15 * X^0"
Reduced form: -5 * X^0 = 0
No solution.
```

### Complex Solutions
```bash
$ ./computor "1 * X^0 + 2 * X^1 + 5 * X^2 = 0"
Reduced form: 1 * X^0 + 2 * X^1 + 5 * X^2 = 0
Polynomial degree: 2
Discriminant is strictly negative, the two complex solutions are:
-1/5 + 2i/5
-1/5 - 2i/5
```

### Quick Test
```bash
make run  # Runs all test cases
```

## 📁 Project Structure
computorv1/
├── Cargo.toml      # Rust project configuration

├── Makefile        # Build automation

└── src/

├──── main.rs         # Entry point

├──── parsing.rs      # Equation parser

├──── math_utils.rs   # Polynomial struct and utilities

└──── solve.rs        # Equation solver logic


## 🔧 Implementation Details

### Parsing Strategy
1. Split equation on `=` to get left and right sides
2. Extract terms in the form `coefficient * X^exponent` | `unit` | `X^exponent`
3. Handle floating-point coefficients and signs and irreductible fraction if necessary
4. Combine same degree terms to create reduced form

### Solving Algorithm
- **Degree 0**: Check if equation is identity or contradiction
- **Degree 1**: Linear equation → `x = -c/b`
- **Degree 2**: Quadratic formula with discriminant check
  - Δ > 0: Two real solutions
  - Δ = 0: One real solution
  - Δ < 0: Two complex conjugate solutions

### Key Rust Concepts Used
- ✅ String parsing and manipulation
- ✅ Error handling with `Result<T, E>`
- ✅ Pattern matching
- ✅ Ownership and borrowing
- ✅ Mutable references
- ✅ Modules and project structure
- ✅ Conditional compilation (`#[cfg(bonus)]`)

## 🎯 Learning Objectives

This project demonstrates:
- **Rust fundamentals**: Coming from C/C++/Python, learning Rust's ownership model and error handling
- **Mathematical programming**: Implementing the quadratic formula and discriminant analysis
- **Parsing**: Building a simple equation parser without external libraries
- **Project organization**: Structuring a multi-file Rust project with modules using Cargo

Made with 🦀 Rust as part of 42's algorithm branch curriculum.

