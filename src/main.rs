use std::env;
mod parsing;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Invalid number of arguments");
        return;
    }
    let expr: String = args[1].clone();
    let res_parsing = parsing::parse_argument(&expr);
    if res_parsing.0 == false {
        println!("{}", res_parsing.1);
        return;
    }
    dbg!(res_parsing.1);
    return;
}
