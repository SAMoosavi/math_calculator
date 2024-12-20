mod parser;

use parser::Expiration;

fn main() {
    let mut a = String::from("- 3 + x * 2 + y ^ (x * 3) + y / 2");

    (1..20)
        .into_iter()
        .for_each(|_| a = format!("({a}) + ({a})"));

    let src = format!("let x = 10; let y = 20; {a}");
    match Expiration::new(&src) {
        Ok(exp) => match exp.calculate() {
            Ok(output) => {
                println!("answer: {}", output)
            }
            Err(eval_err) => println!("Evaluation error: {}", eval_err),
        },
        Err(e) => println!("Parse error: {}", e),
    };
}
