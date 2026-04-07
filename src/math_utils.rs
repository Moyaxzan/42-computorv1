use core::f64;

pub struct Polynomial {
    pub a: f64,
    pub b: f64,
    pub c: f64
}

fn absi(val: i64) -> i64 {
    if val < 0 {
        return -val;
    }
    return val;
}

pub fn absf(val: f64) -> f64 {
    if val < 0.0 {
        return -val;
    }
    return val;
}

fn squaref(val: f64) -> f64 {
    return val * val;
}

// Newton-Raphson method
// x(n+1) = 1/2 * (x(n) + val / x(n))
fn sqrtf(val: f64) -> f64 {
    let epsilon = 0.000001;
    let mut prev_n: f64 = val;
    let mut n: f64 = 0.5 * (val + 1.0);
    while epsilon < absf(prev_n - n) {
        prev_n = n;
        n = 0.5 * (prev_n + val / prev_n);
    }
    return n;
}

pub fn deg_one_solution(px: &Polynomial) -> String {
    if cfg!(bonus) {
        println!("");
        println!("x = -c / b");
        println!("x = -{} / {}", px.c, px.b);
        println!("x = {}", -px.c / px.b);
        println!("");

        let frac = irr_frac(px.c, px.b);
        let s1 = if frac.0 / 10000 > 1 ||  frac.1 / 10000 > 1 {
            format!("{:.6}", -px.c / px.b)
        } else if frac.1 == 1 {
            format!("{}", frac.0)
        } else if frac.1 == -1 {
            format!("{}", -frac.0)
        } else if -px.c / px.b < -f64::EPSILON {
            format!("-{}/{}", frac.0, frac.1)
        } else {
            format!("{}/{}", frac.0, frac.1)
        }
        ;
        return s1;
    }
    return format!("{}", -px.c / px.b);
}

pub fn get_delta(px: &Polynomial) -> f64 {

    let delta = ((squaref(px.b) - (4.0 * px.a * px.c)) * 1_000_000.0).round() / 1_000_000.0;
    if cfg!(bonus) {
        println!("");
        println!("Δ = b^2 - 4ac");
        println!("Δ = {}^2 - 4 * {} * {}", px.b, px.a, px.c);
        println!("Δ = {}", delta);
        println!("");
    }
    return delta;
}

pub fn one_r_solution(px: &Polynomial) -> String {
    let res = -px.b / (2.0 * px.a);
    if cfg!(bonus) {
        println!("");
        println!("x1 = -b / 2a");
        println!("x1 = -{} / (2 * {})", px.b, px.a);
        println!("x1 = {}", res);
        println!("");

        let frac = irr_frac(px.b, 2.0 * px.a);
        let s1 = if frac.0 / 10000 > 1 ||  frac.1 / 10000 > 1 {
            format!("{:.6}", -px.c / px.b)
        } else if frac.1 == 1 {
            format!("{}", frac.0)
        } else if frac.1 == -1 {
            format!("{}", -frac.0)
        } else if -px.c / px.b < -f64::EPSILON {
            format!("-{}/{}", frac.0, frac.1)
        } else {
            format!("{}/{}", frac.0, frac.1)
        }
        ;
        return s1;
    }
    return format!("{}", res);
}

fn my_gcd(first: i64, second:i64) -> i64 {
    if first == 0 {
        return second;
    } else if second == 0 {
        return first;
    }
    if first < second {
        return my_gcd(first, second % first);
    } else {
        return my_gcd(second, first % second)
    }
}

fn dec_to_frac(dval: f64) -> (i64, i64) {
    let int_val = dval.trunc();
    let flt_val = dval - int_val;

    let precision: i64 = 1_000_000;

    let num = (flt_val * precision as f64).round() as i64;
    let gcd = my_gcd(absi(num), absi(precision));

    let num = num / gcd;
    let den = precision / gcd;

    let final_num = (int_val as i64) * den + num;

    return (final_num, den);
}

fn irr_frac(num: f64, den: f64) -> (i64, i64) {
    let num_fact = dec_to_frac(num);
    let num_num: i64 = num_fact.0;
    let num_den: i64 = num_fact.1;

    let den_fact = dec_to_frac(den);
    let den_num: i64 = den_fact.0;
    let den_den: i64 = den_fact.1;

    let gcd = my_gcd(absi(num_num * den_den), absi(num_den * den_num));
    let mut res_num = num_num * den_den / gcd;
    let mut res_den = num_den * den_num / gcd;
    
    if res_den < 0 {
        res_num *= -1;
        res_den *= -1;
    }
    return (res_num, res_den);
}

pub fn r_solutions(px: &Polynomial, delta: &f64) -> String {
    let sqrt_delta = sqrtf(*delta);
    let den = 2.0 * px.a;

    let s1_frac = irr_frac(-px.b + sqrt_delta, den);
    let s2_frac = irr_frac(-px.b - sqrt_delta, den);

    let format_frac = |num: i64, den: i64, val: f64| -> String {
        if num / 10000 > 1 || den / 10000 > 1 {
            format!("{:.6}", val)
        } else if den == 1 {
            format!("{}", num)
        } else if den == -1 {
            format!("{}", -num)
        } else if val < -f64::EPSILON {
            format!("-{}/{}", num, den)
        } else {
            format!("{}/{}", num, den)
        }
    };

    let s1 = format_frac(s1_frac.0, s1_frac.1, (-px.b + sqrt_delta) / den);
    let s2 = format_frac(s2_frac.0, s2_frac.1, (-px.b - sqrt_delta) / den);

    if cfg!(bonus) {
        println!("");
        println!("x1 = (-b + √Δ) / 2a");
        println!("x1 = (-{} + √{}) / (2 * {})", px.b, delta, px.a);
        println!("x1 = {}", s1);
        println!("");
        println!("x2 = (-b - √Δ) / 2a");
        println!("x2 = (-{} - √{}) / (2 * {})", px.b, delta, px.a);
        println!("x2 = {}", s2);
        println!("");
    }

    return format!("{}\n{}", s2, s1)
}

pub fn c_solutions(px: &Polynomial, delta: &f64) -> String {
    let real_frac = irr_frac(-px.b, 2.0 * px.a);
    let comp_frac = irr_frac(sqrtf(absf(*delta)), 2.0 * px.a);
    let real_num = real_frac.0;
    let real_den = real_frac.1;
    let comp_num = comp_frac.0;
    let comp_den = comp_frac.1;

    let real = if real_num / 1000 > 1 || real_den / 1000 > 1 {
        format!("{:.6}", -px.b / 2.0 * px.a)
    } else if real_den == 1 {
        format!("{}", real_num)
    } else if real_den == -1 {
        format!("{}", -real_num)
    } else {
        format!("{}/{}", real_num, real_den)
    };

    let (comp_sign, comp) = if comp_num / 1000 > 1 || comp_den / 1000 > 1 {
        let val = sqrtf(absf(*delta)) / 2.0 * px.a;
        if val < 0.0 { ("-", format!("{:.6}i", -val)) }
        else         { ("+", format!("{:.6}i",  val)) }
    } else if comp_den == 1 {
        if comp_num < 0 { ("-", format!("{}i", -comp_num)) }
        else            { ("+", format!("{}i",  comp_num)) }
    } else if comp_den == -1 {
        if comp_num < 0 { ("+", format!("{}i",  comp_num)) }
        else            { ("-", format!("{}i",  comp_num)) }
    } else {
        if comp_num < 0 { ("-", format!("{}i/{}", -comp_num, comp_den)) }
        else            { ("+", format!("{}i/{}",  comp_num, comp_den)) }
    };

    let neg_sign = if comp_sign == "+" { "-" } else { "+" };
    if cfg!(bonus) {
        println!("");
        println!("x1 = -b / 2a + √Δi / 2a");
        println!("x1 = {} / (2 * {}) + √{}i / (2 * {})", -px.b, px.a, delta, px.a);
        println!("x1 = {} {} {}", real, comp_sign, comp);
        println!("");
        println!("x2 = -b / 2a - √Δi / 2a");
        println!("x2 = {} / (2 * {}) - √{}i / (2 * {})", -px.b, px.a, delta, px.a);
        println!("x2 = {} {} {}", real, neg_sign, comp);
        println!("");
    }
    return format!(
        "{} {} {}\n{} {} {}",
        real, comp_sign, comp,
        real, neg_sign,  comp
    );
}
