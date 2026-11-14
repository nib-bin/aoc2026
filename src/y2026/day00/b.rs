extern crate helpf;

use crate::y2026::day00::DAY;

pub fn b() -> i64 {
    return b_testable(format!("./input/{}/{}.txt", DAY, "b"))
}

fn b_testable(input_path: String) -> i64 {
    if let Ok(lines) = helpf::read_lines(input_path) {
        for line in lines {
            match line {
                Ok(l) => {
                    if l.is_empty() { continue; }

                    println!("{:?}", l);
                },
                Err(e) => println!("Err reading line in {}::{}: {:?}", DAY, "b", e),
            }
        }

    } else {
        println!("cant open file in {}::{}", DAY, "b");
    };

    println!("day00::b");

    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(b_testable(format!("./input/{}/{}test.txt", DAY, "b")), 0);

    }
}
