mod parser;

use std::collections::HashMap;

use parser::Expiration;

fn main() {
    let mut src = String::from("- 3 + x * 2 + y ^ (x * 3) + y / 2");

    (1..5)
        .into_iter()
        .for_each(|_| src = format!("({src}) + ({src})"));

    match Expiration::new(&src) {
        Ok(exp) => match exp.calculate(&HashMap::from([("x", 10 as f64), ("y", 20 as f64)])) {
            Ok(output) => {
                println!("answer: {}", output)
            }
            Err(eval_err) => println!("Evaluation error: {}", eval_err),
        },
        Err(e) => println!("Parse error: {}", e),
    };
}
