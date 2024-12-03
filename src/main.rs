mod parser;

use crate::parser::Expiration;

fn main() {
    let mut a = String::from(" ( 3 + x_2 + 2 ) + ( 2 - x ) - ( [ 3 + x_2 ] + 2 )");
    for _ in 1..22{
        a = format!("( {a} ) + ( {a} )")
    }
    let ex = Expiration::new(a);
    match ex.pars() {
        Ok(_) => {}
        Err(x) => {
            println!("{x}")
        }
    };
}
