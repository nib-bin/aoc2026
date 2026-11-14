extern crate helpf;

mod a;
mod b;

static DAY: &str = "day00";

pub fn a() {
    a::a_testable(format!("./input/{}.txt", DAY));
}

pub fn b() {
    b::b_testable(format!("./input/{}.txt", DAY));
}

fn parse_input(input_path: String) -> i64 {
    if let Ok(lines) = helpf::read_lines(input_path) {
        for line in lines {
            match line {
                Ok(l) => {
                    if l.is_empty() { continue; }

                    println!("{:?}", l);
                },
                Err(e) => println!("Err reading line in {}: {:?}", DAY, e),
            }
        }

    } else {
        println!("cant open file in {}", DAY);
    };

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(a::a_testable(format!("./input/{}test.txt", DAY)), 0);

    }

    #[test]
    fn b() {
        assert_eq!(b::b_testable(format!("./input/{}test.txt", DAY)), 0);

    }
}
