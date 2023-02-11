use try_integration_test::sum;

mod common;
use common::{function_1, function_2};

#[test]
fn sum_test() {
    assert_eq!(sum(4, 6), 10);
}

// cargo test -> without println!(XXX) result
// cargo test -- --nocapture -> with println!(XXX) result
#[test]
fn common_test() {
    function_1();
    assert_eq!(sum(8, 3), 11);
    function_2();
}
