const V: usize = 0xFFFF_FFFF_FFFF_FFFe; //0xF000_0000_0000_0000; //2000000000000000000;
const V2: usize = 0xFFFF_FFFF_FFFF_FFFF; //2000000000000000000;

use qrand_core::{get_sequence, Error, LowDiscrepancySequence};

pub fn rd_u64_par(n: usize, _d: usize) -> u64 {
    let n_: u128 = n as u128;
    println!("n:u128 = {:032x}", n_);
    let alpha: u128 = V2 as u128;
    println!("alpha:u128 = {:032x}", alpha);
    let tmp = n_ * alpha;
    println!("tmp = n_ * alpha = {:032x} ({})", tmp, tmp);
    let value_u128 = tmp >> 64;
    println!("value_u128 = {:032x}", value_u128);
    let value = value_u128 as u64;
    println!("value = {:016x}", value);
    value
}

fn main() {
    let plastic_number = 1.324717957244746025960908854;
    let inverse = 1.0 / plastic_number;
    let inverse_u64_max = inverse * u64::max_value() as f64;
    println!(
        "plastic_number = {:?}. inverse = {:?}. inverse * u64::max = {}",
        plastic_number, inverse, inverse_u64_max
    );
    let seq_quasi_rd = 0.25487766624669295;
    let seq_quasi_rd_u64 = seq_quasi_rd * u64::max_value() as f64;
    println!(
        "seq_quasi_rd = {}. seq_quasi_rd * u64::max = {}",
        seq_quasi_rd, seq_quasi_rd_u64
    ); // 47016630793571_00687 vs. 47016630793571_04000

    let golden_ratio: u64 = (0.7548776662466927 * u64::max_value() as f64) as u64;
    println!("golden_ratio:u64 = {:x}", golden_ratio);

    println!("");

    rd_u64_par(V, 1);

    let sequence = get_sequence();

    for n in 0..10 {
        println!(
            "({}, {})",
            sequence.element(n, 0).unwrap_or(1.1),
            sequence.element(n, 1).unwrap_or(1.1)
        );
    }
}
