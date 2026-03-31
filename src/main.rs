use std::env;
mod parsing;
mod math_utils;

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
    let left = res_parsing.1;
    let right = res_parsing.2;
    let final_polynomial = math_utils::Polynomial{a:left.a - right.a, b:left.b - right.b, c:left.c - right.c};
    //TODO: bad formatting with negative numbers according to the subject
    println!("Reduced form: {} * X^0 - {} * X^1 + {} * X^2 = 0", final_polynomial.c, final_polynomial.b, final_polynomial.a);
    if (final_polynomial.a - 0.0).abs() > f64::EPSILON {
        println!("Polynomial degree: 2");
    } else if (final_polynomial.b - 0.0).abs() > f64::EPSILON {
        println!("Polynomial degree: 1");
    }
    return;
}
