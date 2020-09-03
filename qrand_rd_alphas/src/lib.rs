use proc_macro::TokenStream;

fn calculate_phi(dimension: u32) -> f64 {
    let mut x: f64 = 2.0;
    let power: f64 = f64::from(dimension + 1).recip();
    for i in 0..25 {
        x = x + 1.0;
        x = x.powf(power);
    }
    x
}

fn create_alphas(dimension: u32) -> Vec<f64> {
    let mut vec: Vec<f64> = Vec::with_capacity(dimension as usize);
    let phi = calculate_phi(dimension);
    let inv_g = phi.recip();
    let dim = dimension as usize;
    for i in 0..dimension {
        vec.push(inv_g.powf(f64::from(i + 1)).fract());
    }
    vec
}

#[proc_macro]
pub fn create(dimension: TokenStream) -> TokenStream {
    // Use https://doc.rust-lang.org/std/macro.compile_error.html for error reporting?
    println!("Dimension {}", dimension);
    // let dim_string = dimension.to_string();
    // let dim = dim_string.parse::<u32>().unwrap();
    //let input = parse_macro_input!(dimension as DeriveInput);
    //println!("syn input: {:?}", input);
    let dim = 2;
    let alphas = create_alphas(dim);
    let mut code = String::from("const alphas:[f64;");
    code.push_str(dim.to_string().as_str());
    code.push_str("] = [");
    for i in 0..dim - 1 {
        code.push_str(alphas[i as usize].to_string().as_str());
        code.push_str(", ");
    }
    code.push_str(alphas.last().unwrap().to_string().as_str());
    code.push_str("];");

    println!("{}", code);
    code.parse().unwrap()
}
