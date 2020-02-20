use std::collections::HashMap;

fn main() {
    let s = String::from("hi hello hiromu");
    make_trie(&s);
}

fn make_trie(s: &String) {
    let end = "*";
    let pi: Vec<String> = s
        .split_whitespace()
        .map(|st| {
            st.chars()
                .filter(|ch| ch.is_alphabetic())
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    // for item in pi {
    //     println!("{}", item);
    // }

    for item in pi {
        println!("{}", &item);
        for i in item.to_string().as_str().chars() {
            println!("{}", i);
        }
    }
}
