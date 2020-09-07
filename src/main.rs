mod ast_model;
mod vale_helpers;

use ast_model::ValeCode;
use vale_helpers::build_vale; //, run_vale};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vale_code_simple = ValeCode {
        code: "fn main() { 42 }".into(),
    };
    let vale_code_operation = ValeCode {
        code: "fn main() { 1 + 2 }".into(),
    };
    let vale_code_function = ValeCode {
        code: "fn main() { print(\"hello!\"); }".into(),
    };
    let result_simple = build_vale(vale_code_simple)?;
    let result_operation = build_vale(vale_code_operation)?;
    let result_function = build_vale(vale_code_function)?;

    //println!("{:?}", result_simple);
    //println!("{:?}", result_operation);
    println!("{:?}", result_function);

    Ok(())
}
