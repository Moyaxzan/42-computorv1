use crate::math_utils;

//TODO : out of bound where arg = "0"
fn parse_polynomial(arg: &str, polynomial: &mut math_utils::Polynomial) -> bool {
    let mut mult: f64 = 1.0; //if no sign at beginning -> sign is +
    // let mut coef: f64 = 0.0;

    let mut i: usize = 0;
    let chars: Vec<char> = arg.chars().collect();
    while i < chars.len() {
        if chars[i] == '-' {
            mult = -1.0;
            i += 1;
        } else if chars[i] == '+' {
            mult = 1.0;
            i += 1;
        } else if i != 0 || !chars[i].is_digit(10) { // no sign only allowed for 1st elem
            eprintln!("Invalid Expression1");
            return false;
        }

        // Extract the number 
        let start = i;
        while i < chars.len() && (chars[i].is_digit(10) || chars[i] == '.') {
            i += 1;
        }
        
        if start == i {
            eprintln!("Invalid Expression: Expected number");
            return false;
        }
        
        // Convert char range to string and parse as f64
        let number_str: String = chars[start..i].iter().collect();
        let coef: f64 = match number_str.parse::<f64>() {
            Ok(n) => n * mult,
            Err(_) => {
                eprintln!("Invalid number: {}", number_str);
                return false;
            }
        };

        if chars[i] != '*' {
            eprintln!("Invalid Expression2");
            return false;
        }
        i += 1;
        if chars[i] != 'X' && chars[i] != 'x' {
            eprintln!("Invalid Expression3");
            return false;
        }
        i += 1;
        if chars[i] != '^' {
            eprintln!("Invalid Expression4");
            return false;
        }
        i += 1;
        if !chars[i].is_digit(10) {
            eprintln!("Invalid Expression5");
            return false;
        } else if chars[i] > '2' {
            //TODO: fix bad format according to the subject
            eprintln!("Polynomial degree: {}", chars[i]);
            eprintln!("The polynomial degree is strictly greater than 2, I can't solve.Invalid Expression");
            return false;
        }
        if chars[i] == '0' {
            polynomial.c = coef;
        } else if chars[i] == '1' {
            polynomial.b = coef;
        } else if chars[i] == '2' {
            polynomial.a = coef;
        }
        i += 1
    }
    return true;
}

pub fn parse_argument(arg: &String) -> (bool, math_utils::Polynomial, math_utils::Polynomial)
{

    let mut left: math_utils::Polynomial = math_utils::Polynomial{a:0.0, b:0.0, c:0.0};
    let mut right: math_utils::Polynomial = math_utils::Polynomial{a:0.0, b:0.0, c:0.0};


    let trimmed = arg.trim().replace(" ", "");

    let cleaned = trimmed.split_once('=');
    match cleaned {
        Some((left_str, right_str)) => {
            if !parse_polynomial(left_str, &mut left) || !parse_polynomial(right_str, &mut right) {
                return (false, left, right);
            }
        }
        None => {
            eprintln!("Expression must have an = symbol");
            return (false, left, right);  // No '=' found
        }
    }
    return (true, left, right);
}
