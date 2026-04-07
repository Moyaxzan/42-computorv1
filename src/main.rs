mod math_utils;
mod parsing;
mod solve;
use std::env;
use std::io::{self, Read};


fn main() {
    let args: Vec<String> = env::args().collect();

    let mut expr: String = String::new();
    if args.len() > 2 {
        println!("Invalid number of arguments");
        return;
    } else if args.len() == 1 { // Read STDIN when no arguments given
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_to_string(&mut expr) {
            Ok(n) => {
                if n <= 0 {
                    println!("Error while reading STDIN");
                }
            }
            Err(error) => println!("Error {} while reading STDIN", error),
        }
    } else {
        expr = args[1].clone();
    }
    let res_parsing = parsing::parse_argument(&expr);
    if res_parsing.0 == false {
        return;
    }
    let left: [f64; 10] = res_parsing.1;
    let right: [f64; 10] = res_parsing.2;

    // Get reduced form
    let reduced_polynomial: [f64; 10] = [
        left[0] - right[0],
        left[1] - right[1],
        left[2] - right[2],
        left[3] - right[3],
        left[4] - right[4],
        left[5] - right[5],
        left[6] - right[6],
        left[7] - right[7],
        left[8] - right[8],
        left[9] - right[9]
    ];

    // Get degree of polynomial
    let mut index: usize = 0;
    let mut degree: i64 = 0;
    while index <= 9 {
        if math_utils::absf(reduced_polynomial[index]) > f64::EPSILON {
            degree = index as i64;
        }
        index += 1;
    }

    // Print
    print_reduced_form(reduced_polynomial, degree as usize);
    if degree != 0 {
        println!("Polynomial degree: {}", degree);
    }
    if degree > 2 {
        println!("The polynomial degree is strictly greater than 2, I can't solve.");
        return;
    }
    
    let polynomial = math_utils::Polynomial{
        a: reduced_polynomial[2],
        b: reduced_polynomial[1],
        c: reduced_polynomial[0]
    };

    solve::solve(&polynomial);

    return;
}

fn print_reduced_form(reduced_polynomial: [f64; 10], degree: usize) {
    print!("Reduced form: ");
    let mut index: usize = 0;
    while index <= degree {
        if reduced_polynomial[index] < 0.0 {
            if index == 0 {
                print!("-");
            } else {
                print!("- ");
            }
        } else if index != 0 {
            print!("+ ");
        }
        print!("{} * X^{} ", math_utils::absf(reduced_polynomial[index]), index);
        index += 1;
    }
    println!("= 0");
}
