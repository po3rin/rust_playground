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
    println!("answer {}", answer);

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

    // ownership
    string();
    ownership();
    string_print();
    dangle()

}

fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // v1とv2についての作業を行う

    // 答えを返す
    v1[0] + v2[1]
}

fn string(){
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える
    println!("{}", s); // これは`hello, world!`と出力する

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn ownership() {
    let s1 = gives_ownership();         // gives_ownershipは、戻り値をs1に
                                        // ムーブする
    let s2 = String::from("hello");     // s2がスコープに入る

    let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ
                                        // 戻り値もs3にムーブされる
} // ここで、s3はスコープを抜け、ドロップされる。s2もスコープを抜けるが、ムーブされているので、
  // 何も起きない。s1もスコープを抜け、ドロップされる。

fn gives_ownership() -> String {             // gives_ownershipは、戻り値を
                                             // 呼び出した関数にムーブする

    let some_string = String::from("hello"); // some_stringがスコープに入る

    some_string                              // some_stringが返され、呼び出し元関数に
                                             // ムーブされる
}

// takes_and_gives_backは、Stringを一つ受け取り、返す。
fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る。

    a_string  // a_stringが返され、呼び出し元関数にムーブされる
}

fn string_print(){
    let mut s = String::from("hello");
    let (len, se) = calculate_length(&mut s);
    println!("{}, {}, {}", s, len, se);
}

fn calculate_length(s: &mut String) -> (usize, String) {
    s.push_str(", world");
    (s.len(), s.to_string())
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}