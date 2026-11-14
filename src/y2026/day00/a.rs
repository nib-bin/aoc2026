use super::{DAY, parse_input};

pub fn a_testable(input_path: String) -> i64 {
    let zero = parse_input(input_path);

    println!("{}::a: {}", DAY, zero);

    zero
}
