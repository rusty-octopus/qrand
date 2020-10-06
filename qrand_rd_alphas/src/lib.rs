fn calculate_phi(dimension: usize) -> f64 {
    let mut phi: f64 = 2.0;
    let power: f64 = f64::from(dimension as u32 + 1).recip();
    for _ in 0..25 {
        phi += 1.0;
        phi = phi.powf(power);
    }
    phi
}

pub fn create(dimension: usize) -> Box<[f64]> {
    let mut alphas: Vec<f64> = Vec::with_capacity(dimension);
    let phi = calculate_phi(dimension);
    let inv_g = phi.recip();
    let dim = dimension as u32;
    for i in 0..dim {
        alphas.push(inv_g.powf(f64::from(i + 1)).fract());
    }
    alphas.into_boxed_slice()
}
