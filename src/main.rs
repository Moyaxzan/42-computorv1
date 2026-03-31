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
    dbg!(left.a, left.b, left.c);
    dbg!(right.a, right.b, right.c);
    return;
}
