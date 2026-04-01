use core::f64;
use std::fmt::Debug;


pub struct Polynomial {
    pub a: f64,
    pub b: f64,
    pub c: f64
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

pub fn deg_one_solution(px: &Polynomial) -> f64 {
    return -px.c / px.b;
}

pub fn get_delta(px: &Polynomial) -> f64 {
    return squaref(px.b) - (4.0 * px.a * px.c);
}

pub fn one_r_solution(px: &Polynomial) -> f64 {
    return -px.b / (2.0 * px.a);
}

fn my_gcd(first: f64, second:f64) -> f64 {
    if absf(first) < f64::EPSILON || absf(second) < f64::EPSILON {
        return 1.0;
    }
    let mut x = first;
    let mut y = second;
    while absf(y) > f64::EPSILON {
        if y.round() < x.round() {
            std::mem::swap(&mut x, &mut y);
        }
        y = y.round() % x.round();
    }
    return x.round();
}

fn my_fmt(val: f64) -> String {
    let rounded = (val * 1_000_000.0).round() / 1_000_000.0;
    return format!("{}", rounded);
}

pub fn c_solutions(px: &Polynomial, delta: &f64) -> String {
    let mut real_numerator: f64 = - px.b;
    let mut real_denominator: f64 = 2.0 * px.a;
    let mut comp_numerator: f64 = sqrtf(absf(*delta));
    let mut comp_denominator: f64 = 2.0 * px.a;

    let real_gcd = my_gcd(absf(real_numerator), absf(real_denominator));
    let comp_gcd = my_gcd(absf(comp_numerator), absf(comp_denominator));

    real_numerator /= real_gcd;
    real_denominator /= real_gcd;
    comp_numerator /= comp_gcd;
    comp_denominator /= comp_gcd;
    return format!(
        "{}/{} + {}i/{}\n{}/{} - {}i/{}",
        my_fmt(real_numerator),
        my_fmt(real_denominator),
        my_fmt(comp_numerator),
        my_fmt(comp_denominator),
        my_fmt(real_numerator),
        my_fmt(real_denominator),
        my_fmt(comp_numerator),
        my_fmt(comp_denominator)
    );
}
