use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use num_traits::pow::Pow;
use std::thread;

fn main() {
    println!("{}", ackermann(4, 2));
}

fn b(v: u32) -> BigUint {
    BigUint::from(v)
}

fn naive_ackermann(m: i32, n: i32) -> i32 {
    fn work(m: i32, n: i32) -> i32 {
        if m == 0 {
            n + 1
        } else if n == 0 {
            work(m - 1, 1)
        } else {
            work(m - 1, work(m, n - 1))
        }
    }

    fn run_in_thread(m: i32, n: i32) -> i32 {
        let builder = thread::Builder::new()
            .name("large_stack_thread".into())
            .stack_size(32 * 1024 * 1024);

        let handler = builder.spawn(move || work(m, n)).unwrap();

        handler.join().unwrap()
    }

    run_in_thread(m, n)
}

fn ackermann(m: u32, n: u32) -> BigUint {
    fn work(m: BigUint, n: BigUint) -> BigUint {
        match m.to_u32().unwrap() {
            0 => n + b(1),
            1 => n + b(2),
            2 => b(2) * n + b(3),
            3 => {
                // 指数部はBigUintは使えない: https://github.com/rust-num/num-bigint/pull/54/files
                let exp = (n + b(3)).to_u128().unwrap();
                b(2).pow(exp) - b(3)
            }
            _ => {
                if m == b(0) {
                    n + b(1)
                } else if n == b(0) {
                    work(m - b(1), b(1))
                } else {
                    work(m.clone() - b(1), work(m, n - b(1)))
                }
            }
        }
    }

    work(b(m), b(n))
}

// fn ackermann(m: i32, n: i32) -> i32 {
//     if m == 0 {
//         n + 1
//     } else if n == 0 {
//         ackermann(m - 1, 1)
//     } else {
//         ackermann(m - 1, ackermann(m, n - 1))
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    // use std::convert::TryInto;

    #[test]
    fn a4() {
        assert_eq!(ackermann(4, 1), BigUint::new(vec![65533]));
    }

    #[test]
    fn conv_from_u32_biguint() {
        assert_eq!(b(5), BigUint::new(vec![5]));
    }
}
