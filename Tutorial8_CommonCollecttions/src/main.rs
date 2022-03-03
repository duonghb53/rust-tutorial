use std::collections::HashMap;
use std::fmt::Debug;

fn main() {
    let mut vec: Vec<i32> = Vec::new();

    vec.push(2);
    vec.push(3);
    vec.push(4);

    let mut v = vec![1, 2, 3];
    v.push(4);

    let third = v[2];

    v.push(5);

    println!("The third element is {}", third);

    match v.get(3) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no element"),
    }

    //let does_not_exist = v[100];
    let does_not_exist = v.get(100);

    for i in &mut v {
        *i += 1;
        println!("{}", i);
    }

    string_common();
    hash_map();
    count_word();

    let row = vec![
        SheetCell::Int(5),
        SheetCell::Float(10.15),
        SheetCell::Text(String::from("Hello")),
    ];

    match &row[2] {
        SheetCell::Float(i) => println!("{}", i),
        _ => println!("This is not Float"),
    }

    // println!("{:#?}", row[2]); // Error
}

enum SheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn string_common() {
    let mut s = String::new();

    let s = "initial strings".to_owned();
    let mut s = "initial strings".to_string();
    s.push('v');
    s.push_str("Xin chaof");

    println!("{}", s);

    //let h = &s[0];
}


fn hash_map() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    println!("{:#?}", scores);
    for (key, value) in scores {
        println!("{}: {}", key, value);
    }
}

fn count_word() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        //println!("{:?}", map);
        *count += 1;
        println!("count: {:?}", count);
    }
    println!("{:?}", map);
}