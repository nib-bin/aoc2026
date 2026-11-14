use super::{DAY, parse_input};

pub fn b_testable(input_path: String) -> i64 {
    let zero = parse_input(input_path);

    println!("{}::b: {}", DAY, zero);

    zero
}
