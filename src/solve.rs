use crate::math_utils;

pub fn solve(px: &math_utils::Polynomial) -> bool {

    if px.a.abs() < f64::EPSILON { // degree < 2
        if px.b.abs() < f64::EPSILON { // degree 0 equation
            if px.c.abs() < f64::EPSILON {
                println!("Any real number is a solution.");
                return true;
            } else {
                println!("No solution.");
                return false;
            }
        } else { // degree 1 equation
            println!("The solution is:");
            println!("{}", math_utils::deg_one_solution(px));
            return true;
        }
    }

    let delta = math_utils::get_delta(px);

    if delta > f64::EPSILON {
        println!("Discriminant is stricly positive, the two solutions are:");
        println!("{}", math_utils::r_solutions(px, &delta));
    } else if delta < -(f64::EPSILON) {
        println!("Discriminant is stricly negative, the two complex solutions are:");
        println!("{}", math_utils::c_solutions(px, &delta));
    } else {
        println!("Discriminant is null, the solution is:");
        println!("{}", math_utils::one_r_solution(px));
    }
    return true;
}
