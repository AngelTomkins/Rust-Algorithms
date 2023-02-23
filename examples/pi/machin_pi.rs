// Machin formula

fn main() {
    println!("Using Machin formula: pi = {}", calculate_pi_machin(0));
}


// Due to limitations of f64 precision, this calculates maximum precision of 14 digits of pi
pub fn calculate_pi_machin(precision: usize) -> f64 {
    let mut pi = 0.0;
    let mut last_pi = 1.0;
    let precisionf: f64 = if precision != 0 {
        0.1_f64.powi(precision as i32+5)
    } else {
        0.00000000000001
    };

    let mut n = 0;
    let arctan = |x: f64, n: &i32| -> f64 {
        ((-1.0_f64).powi(*n) / (2.0 * *n as f64 + 1.0)) * x.powi(2 * *n + 1)
    };

    loop {
        pi += 4.0 * arctan(1.0/5.0, &n) - arctan(1.0/239.0, &n);
        n += 1;

        if (pi - last_pi).abs() < precisionf {
            if precision != 0 {
                return 4.0 * (pi * 10_f64.powi(precision as i32)).round()/10_f64.powi(precision as i32);
            } else {
                return 4.0 * pi;
            }
        }
        last_pi = pi;

    }
}
