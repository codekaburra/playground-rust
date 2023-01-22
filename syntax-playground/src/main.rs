#![allow(unreachable_code)]
mod try_format_print;
mod try_loop;
#[warn(unused_labels)]
mod try_match;
use try_match::Color;
fn main() {
    math();
    tup();
    tuple();
    array();

    let yy = {
        let x = 3;
        x + 1 // Expressions do not include ending semicolons
              // If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
    };
    println!("The value of yy is: {}", yy);
    if_else();
    try_format_print::format_print();
    closures();
    try_loop::loop_forever();
    try_loop::loop_number_range();
    try_loop::loop_break_with_return();
    try_loop::loop_nesting_label();
    try_loop::loop_while();
    try_loop::loop_iterator();
    try_match::match_number();
    try_match::match_bool();
    try_match::match_tuple();
    try_match::match_array_slice();
    try_match::match_enum(Color::RGB(122, 17, 40));
    try_match::match_enum(Color::CMY(122, 17, 40));
    try_match::match_blinding();
    try_match::match_if_let();
    try_match::match_loop_let();
    println!("sum is {}", sum(8, 9));
    let val: u64 = 8;
    increase(val, 10);
    println!("val after increase() {}", val); // val unchange outside increase()
}

fn closures() {
    println!("--- closures() ---");
    let doubler = |x| x * 2;
    let value = 5;
    let twice = doubler(value);

    println!("{} doubled is {}", value, twice);
}

fn math() {
    println!("--- math() ---");
    let sum = 5 + 10;
    println!("sum {}", sum);

    let difference = 95.5 - 4.3;
    println!("difference {}", difference);

    let product = 4 * 30;
    println!("product {}", product);

    let quotient = 56.7 / 32.2;
    println!("quotient {}", quotient);
    let floored = 2 / 3;
    println!("floored {}", floored);

    let remainder = 43 % 5;
    println!("remainder {}", remainder);
}

fn sum(a: u64, b: u64) -> u64 {
    a + b // Note: ending without ;  -> same as return a + b;
}
fn increase(mut value: u64, increaseBy: u64) { // Note: mut for value
    value+=increaseBy;
    println!("increase to {}", value);
}

fn tup() {
    println!("--- tup() ---");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;
    let (a, b, c) = (200, 5.4, 7);

    println!("The value of z,y,x is: {} {} {}", z, y, x);
    println!("The value of c,b,a is: {} {} {}", c, b, a);
    {
        let (x, y, z) = (a, b, c);
        println!("inner - The value of z,y,x is: {} {} {}", z, y, x);
    }
    println!("outer - The value of z,y,x is: {} {} {}", z, y, x);
}

fn tuple() {
    println!("--- tuple() ---");
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let tuple_first_element = tuple.0;
    println!("tuple_first_element {}", tuple_first_element);

    let tuple_seconds_element = tuple.1;
    println!("tuple_seconds_element {}", tuple_seconds_element);

    let tuple_third_element = tuple.2;
    println!("tuple_third_element {}", tuple_third_element);
}

fn array() {
    println!("--- array() ---");

    let array1 = [11, 12, 13, 14, 15];
    let array2: [i32; 5] = [1, 2, 3, 4, 5];

    println!("array1 length {}", array1.len());

    let array1_element_0 = array1[0];
    println!("array1_element_0 {}", array1_element_0);

    for n in array2 {
        println!("{}", n);
    }
}
fn if_else() {
    println!("--- if_else() ---");
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        // This expression returns an `i32`.
        10 * n
    } else {
        println!(", and is a big number, halve the number");

        // This expression must return an `i32` as well.
        n / 2 // Try suppressing this expression with a semicolon.
    };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);
}
