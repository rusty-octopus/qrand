// used to generate the alpha values
#[cfg(feature = "rd")]
#[cfg(not(feature = "sobol"))]
#[cfg(not(feature = "std_interface"))]
use qrand_rd_alphas::create;
// primarily used for writing the file
use std::{env, fs, path::Path};

fn create_rd_alphas(dimension: usize) -> String {
    // create string from the alpha values
    let mut array_string = String::from("static ALPHAS:[f64; ");
    array_string.push_str(dimension.to_string().as_str());
    array_string.push_str("] = [\r\n");
    let alphas = create(dimension);
    for alpha in &alphas {
        array_string.push_str("\u{20}\u{20}\u{20}\u{20}");
        array_string.push_str(alpha.to_string().as_str());
        array_string.push_str(",\r\n");
    }
    array_string.push_str("];\r\n");
    array_string
}

fn main() {
    let is_std_interface = env::var("CARGO_FEATURE_STD_INTERFACE").is_ok();

    if !is_std_interface {
        // get the DIMENSION environment variable or panic
        let dimension = usize::from_str_radix(env::var("DIMENSION").unwrap().as_str(), 10).unwrap();

        // create string from the alpha values
        let mut array_string = String::from("static ALPHAS:[f64; ");
        array_string.push_str(dimension.to_string().as_str());
        array_string.push_str("] = [\r\n");
        let alphas = create(dimension);
        for alpha in &alphas {
            array_string.push_str("\u{20}\u{20}\u{20}\u{20}");
            array_string.push_str(alpha.to_string().as_str());
            array_string.push_str(",\r\n");
        }
        array_string.push_str("];\r\n");

        // write the string to a file
        let out_dir = env::var("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("alphas.rs");
        fs::write(&dest_path, array_string).unwrap();
    }

    // set reasons to rebuild
    println!("cargo:rerun-if-env-changed=DIMENSION");
}
