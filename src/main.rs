mod parser;

use crate::parser::Expiration;

fn main() {
    let ex = Expiration::new("(([3+x_2]+2)+(2-x))-5");
    match ex.pars() {
        Ok(_) => {}
        Err(x) => {
            println!("{x}")
        }
    };
}
