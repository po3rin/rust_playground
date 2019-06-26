struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 { self.x }
}

fn main() {
    // 借用
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let answer = foo(&v1, &v2);
    println!("{}", answer);

    // &mut参照
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("{}", x);

    // イテレータの無効
    let v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
    }

    let y = &5; // これは`let _y = 5; let y = &_y;`と同じ
    let f = Foo { x: y };

    println!("x is: {}", f.x());

    let mut m = 4;
    m = 6; // 問題なし!
    println!("m is: {}", m);

    let mut m2 = 5;
    m2 = 3;
    let mm = &mut m2;
    *mm = 10;
    println!("mm is: {}", mm);

}

fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // v1とv2についての作業を行う

    // 答えを返す
    v1[0] + v2[1]
}

