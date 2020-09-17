use std::fmt::Debug;
use std::fmt::Display;

#[derive(Debug)]
pub enum QrandBuildScriptErrors {
    MissingEnvironmentVariable(String),
    InvalidEnvironmentVariableValue(String),
}

impl QrandBuildScriptErrors {
    fn create_missing_environment_variable(name: &str) -> Self {
        Self::MissingEnvironmentVariable(format!("Environment variable \"{}\" is missing.", name))
    }

    fn create_invalid_environment_variable_value(name: &str, type_string: &str) -> Self {
        Self::MissingEnvironmentVariable(format!(
            "Environment variable \"{}\" must be a valid `{}` value.",
            name, type_string
        ))
    }
}

impl Display for QrandBuildScriptErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            Self::MissingEnvironmentVariable(error_message) => error_message,
            Self::InvalidEnvironmentVariableValue(error_message) => error_message,
        };
        write!(f, "{}", error_message)
    }
}

fn parse_environment_variable_value<T, E: Debug, F: Fn(&str) -> Result<T, E>>(
    name: &str,
    type_string: &str,
    construct: F,
) -> Result<T, QrandBuildScriptErrors> {
    use std::env;
    let env_result = env::var(name);
    if env_result.is_ok() {
        let value_result = construct(env_result.unwrap().as_str());
        if value_result.is_ok() {
            Ok(value_result.unwrap())
        } else {
            Err(
                QrandBuildScriptErrors::create_invalid_environment_variable_value(
                    name,
                    type_string,
                ),
            )
        }
    } else {
        Err(QrandBuildScriptErrors::create_missing_environment_variable(
            name,
        ))
    }
}

#[cfg(feature = "rd")]
#[cfg(not(feature = "sobol"))]
#[cfg(not(feature = "std_interface"))]
fn create_rd_alphas() -> Result<(), QrandBuildScriptErrors> {
    // used to generate the alpha values
    use qrand_rd_alphas::create;

    // primarily used for writing the file
    use std::{env, fs, path::Path};

    // get the DIMENSION environment variable
    let dimension_result =
        parse_environment_variable_value("DIMENSION", "usize", |dimension_string| {
            usize::from_str_radix(dimension_string, 10)
        });
    if dimension_result.is_ok() {
        let dimension = dimension_result.unwrap();

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
        Err(dimension_result.unwrap_err())
    }
}

fn main() -> Result<(), QrandBuildScriptErrors> {
    #[cfg(feature = "rd")]
    #[cfg(not(feature = "sobol"))]
    #[cfg(not(feature = "std_interface"))]
    create_rd_alphas()?;

    // set reasons to rebuild
    println!("cargo:rerun-if-env-changed=DIMENSION");

    Ok(())
}
