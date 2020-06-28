use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    println!("OUT_DIR={:?}", out_dir);
    let dest_path = Path::new(&out_dir).join("alphas.rs");
    fs::write(
        &dest_path,
        "static ALPHAS:[f64;2] = [0.7548776662466927, 0.5698402909980532];
        "
    ).unwrap();
}