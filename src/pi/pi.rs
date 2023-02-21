// Pi Calculation Algorithms


// Leibniz Formula
#[allow(dead_code)]
pub fn calculate_pi_leibniz(precision: usize) -> f64 {
    let mut pi = 0.0;
    let mut last_pi = 1.0;
    let precisionf: f64 = 0.1_f64.powi(precision as i32+5);

    let mut n = 0;
    loop {
        for _ in 0..100 {
            pi += (-1_f64).powi(n) * (1.0/(2.0 * n as f64 + 1.0));
            n += 1;
        }

        if (pi - last_pi).abs() < precisionf {
            return (4.0 * pi * 10_f64.powi(precision as i32)).round()/10_f64.powi(precision as i32);
        }
        last_pi = pi;
    }
}
