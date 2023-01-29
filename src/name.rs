use std::env;

/*
Under same directory
Run
    rustc name.rs
    ./name <input the name>
*/

fn main() {
    let name = env::args().skip(1).next();
    match name {
        Some(n) => println!("Hi there!{}", n),
        None => panic!("didnt's receive any name ? "),
    }
}
