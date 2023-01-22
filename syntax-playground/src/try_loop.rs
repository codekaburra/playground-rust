#![allow(unreachable_code)]
#[warn(unused_labels)]
pub fn loop_forever() {
    println!("--- loop_forever() ---");

    let mut count = 0u32;

    println!("Let's count until infinity! {}", 0u32);

    // Infinite loop
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // Skip the rest of this iteration
            continue;
        }
        println!("{}", count);
        if count == 5 {
            // Exit this loop
            break;
        }
    }
}

pub fn loop_number_range() {
    println!("--- loop_number_range() ---");
    for n in 0..23 {
        println!("{}", n);
    }
}

pub fn loop_nesting_label() {
    println!("--- loop_nesting_label() ---");
    'outer: loop {
        println!("Entered the outer loop");

        // need #[warn(unused_labels)]
        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
pub fn loop_break_with_return() {
    println!("--- loop_break_with_return() ---");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

pub fn loop_while() {
    println!("--- loop_while() ---");

    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        println!("{}", n);
        n += 1;
    }
}

pub fn loop_iterator() {
    println!("--- loop_iterator() ---");
    // by default the for loop will apply the into_iter function

    /*
     * iter()  like foreach
     *  This borrows each element of the collection through each iteration.
     *  Thus leaving the collection untouched and available for reuse after the loop
     */
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);

    /*
     * into_iter() like reduce
     *  This borrows each element of the collection through each iteration.
     *  Thus leaving the collection untouched and available for reuse after the loop
     */
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("names: {:?}", names); // <--- will error

    /*
     * iter_mut() like map
     *  This mutably borrows each element of the collection,
     *  allowing for the collection to be modified in place.
     */
    let mut names = vec!["Bob", "Frank", "Ferris"]; // have to mut

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
