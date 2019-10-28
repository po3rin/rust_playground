fn main() {
    println!("{}", ackermann(1, 0));
}

fn ackermann(m: i32, n: i32) -> i32 {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ackermann(m - 1, 1)
    } else {
        ackermann(m - 1, ackermann(m, n - 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn a0() {
        for i in 0..10 {
            assert_eq!(ackermann(0, i), i + 1);
        }
    }

    #[test]
    fn a1() {
        for i in 0..10 {
            assert_eq!(ackermann(1, i), i + 2);
        }
    }

    #[test]
    fn a2() {
        for i in 0..10 {
            assert_eq!(ackermann(2, i), 2 * i + 3);
        }
    }

    #[test]
    fn a3() {
        for i in 0..10 {
            let ui: u32 = i.try_into().unwrap();
            assert_eq!(ackermann(3, i), 2_i32.pow(ui + 3) - 3);
        }
    }
}
