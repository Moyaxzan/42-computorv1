mod math_utils;
mod parsing;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Invalid number of arguments");
        return;
    }
    let expr: String = args[1].clone();
    let res_parsing = parsing::parse_argument(&expr);
    if res_parsing.0 == false {
        return;
    }
    let left: [f64; 10] = res_parsing.1;
    let right: [f64; 10] = res_parsing.2;

    // Reduced form
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
    print_reduced_form(reduced_polynomial);

    // Degree of polynomial
    let mut index: usize = 0;
    let mut degree: i64 = 0;
    while index <= 9 {
        if left[index].abs() > f64::EPSILON || right[index].abs() > f64::EPSILON {
            degree = index as i64;
        }
        index += 1;
    }
    println!("Polynomial degree: {}", degree);
    return;
}

fn print_reduced_form(reduced_polynomial: [f64; 10]) {
    print!("Reduced form: ");
    let mut index: usize = 0;
    while index < 10 {
        if reduced_polynomial[index].abs() > f64::EPSILON {
            if reduced_polynomial[index] < 0.0 {
                print!("- ");
            } else if index != 0 {
                print!("+ ");
            }
            print!("{} * X^{} ", reduced_polynomial[index].abs(), index);
        }
        index += 1;
    }
    println!("= 0");
}
