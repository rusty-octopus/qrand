#[cfg(feature = "rd")]
#[cfg(not(feature = "sobol"))]
#[cfg(not(feature = "std_interface"))]
fn create_sequence_data() -> Result<(), ()> {
    // used to generate the alpha values
    use qrand_rd_alphas::create;

    // primarily used for writing the file
    use std::{env, fs, path::Path};

    // get the DIMENSION environment variable
    let dimension_result = env::var("DIMENSION");
    if dimension_result.is_ok() {
        let dimension_parse_result = usize::from_str_radix(dimension_result.unwrap().as_str(), 10);
        if dimension_parse_result.is_ok() {
            let dimension = dimension_parse_result.unwrap();

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
            Ok(())
        } else {
            eprintln!("Build script error: Please specify a valid usize value as \"DIMENSION\" environment variable");
            Err(())
        }
    } else {
        eprintln!("Build script error: Please specify the \"DIMENSION\" environment variable with a valid usize value.");
        Err(())
    }
}

fn main() {
    // set reasons to rebuild
    println!("cargo:rerun-if-env-changed=DIMENSION");

    #[cfg(feature = "rd")]
    #[cfg(not(feature = "sobol"))]
    #[cfg(not(feature = "std_interface"))]
    let result = create_sequence_data();
    #[cfg(feature = "rd")]
    #[cfg(not(feature = "sobol"))]
    #[cfg(not(feature = "std_interface"))]
    if result.is_err() {
        std::process::exit(1);
    }
}
