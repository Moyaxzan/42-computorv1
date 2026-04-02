// TODO : bad behavior -> ./computor "5 + 4 * X^1 + 3* X^2= 2 * X^2"
fn parse_polynomial(arg: &str, coefs: &mut [f64; 10]) -> bool {
    let mut mult: f64 = 1.0; //if no sign at beginning -> sign is +

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
            eprintln!("Invalid Expression: Expected sign");
            return false;
        }

        // Extract the number 
        let start = i;
        while i < chars.len() && (chars[i].is_digit(10) || chars[i] == '.') {
            i += 1;
        }
        
        if start == i {
            //TODO parse X^n
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

        if i < chars.len() && chars[i] == '*' {
            i += 1;
            if i == chars.len() || chars[i] != 'X' && chars[i] != 'x' {
                eprintln!("Invalid Expression: Expected X");
                return false;
            }
            i += 1;
            if i == chars.len() || chars[i] != '^' {
                coefs[1] += coef;
            } else {
                i += 1;
                if i == chars.len() || !chars[i].is_digit(10) {
                    eprintln!("Invalid Expression: Expected exponent");
                    return false;
                }
                
                coefs[(chars[i] as usize) - ('0' as usize)] += coef;
                i += 1
            }
        } else if i <= chars.len() {
            coefs[0] += coef;
        }
    }
    return true;
}

pub fn parse_argument(arg: &String) -> (bool, [f64; 10], [f64; 10])
{

    let mut left_coefs: [f64; 10] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let mut right_coefs: [f64; 10] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];


    let trimmed = arg.trim().replace(" ", "");

    let cleaned = trimmed.split_once('=');
    match cleaned {
        Some((left_str, right_str)) => {
            if !parse_polynomial(left_str, &mut left_coefs) || !parse_polynomial(right_str, &mut right_coefs) {
                return (false, left_coefs, right_coefs);
            }
        }
        None => {
            eprintln!("Expression must have an = symbol");
            return (false, left_coefs, right_coefs);  // No '=' found
        }
    }
    return (true, left_coefs, right_coefs);
}
