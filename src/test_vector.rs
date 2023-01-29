fn main() {
    let mut numbers_vec: Vec<u8> = Vec::new();
    numbers_vec.push(1);
    numbers_vec.push(2);

    let mut vec_with_macro = vec![1];
    vec_with_macro.push(2);

    checking(&numbers_vec, &vec_with_macro);

    let vec_with_macro_2 = vec![1, 2];
    checking(&vec_with_macro_2, &vec_with_macro);

    let _ = vec_with_macro.pop();

    checking(&numbers_vec, &vec_with_macro);
}

fn checking(numbers_vec: &Vec<u8>, vec_with_macro: &Vec<u8>) {
    let message = if numbers_vec == vec_with_macro {
        "same"
    } else {
        "different"
    };
    println!("{} {:?} {:?}", message, numbers_vec, vec_with_macro);
}
