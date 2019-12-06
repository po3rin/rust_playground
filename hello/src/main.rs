fn cf(x: &i32) -> i32 {
    x + 1
}

fn main() {
    // let cl = |x| x + 1;
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(cf).collect();

    println!("{:?}", v2)
}
