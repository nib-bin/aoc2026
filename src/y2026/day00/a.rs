extern crate helpf;

use crate::y2026::day00::DAY;

pub fn a() -> i64 {
    return a_testable(format!("./input/{}/{}.txt", DAY, "a"))
}

fn a_testable(input_path: String) -> i64 {
    if let Ok(lines) = helpf::read_lines(input_path) {
        for line in lines {
            match line {
                Ok(l) => {
                    if l.is_empty() { continue; }

                    println!("{:?}", l);
                },
                Err(e) => println!("Err reading line in {}::{}: {:?}", DAY, "a", e),
            }
        }

    } else {
        println!("cant open file in {}::{}", DAY, "a");
    };

    println!("day00::a");

    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(a_testable(format!("./input/{}/{}test.txt", DAY, "a")), 0);

    }
}
