use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    println!("OUT_DIR={:?}", out_dir);
    let dimension = usize::from_str_radix(
        env::var_os("CARGO_CFG_DIMENSION")
            .unwrap()
            .into_string()
            .unwrap()
            .as_str(),
        10,
    )
    .unwrap();
    println!("DIMENSION={:?}", dimension);
    let mut array_string = String::from("static ALPHAS:[f64;");
    array_string.push_str(dimension.to_string().as_str());
    array_string.push_str("] = [0.7548776662466927, 0.5698402909980532");
    for i in 2..dimension {
        array_string.push_str(", 0.");
        array_string.push_str(i.to_string().as_str());
    }
    array_string.push_str("];");
    let dest_path = Path::new(&out_dir).join("alphas.rs");
    fs::write(&dest_path, array_string).unwrap();

    // set reasons to rebuild
    println!("cargo:rerun-if-env-changed=CARGO_CFG_DIMENSION");
}
