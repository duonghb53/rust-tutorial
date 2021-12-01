fn main() {
    //let s = String::from("hello");
    //let s1 = s;
    //println!("{}", s1);
    //println!("{}", s);

    //let mut s = String::from("hello");
    //let mut s1 = &mut s;
    //s1.push_str("world 1");
    //s.push_str("world");
    //s.push_str(" world");
    //println!("{} {}", s1, s);
    //println!("{}", s);
    //println!("{}", s1);

    //let s = String::from("hello");
    //let s1 = s;


    //let mut s = String::from("hello");

    //let r1 = &s;
    //println!("{}", r1);
    //let r2 = &s;
    //let r3 = &mut s;
    //println!("{}", r2);
    //println!("{}", r1);
    //println!("{}", s);
    //r3.push_str(" NFT!");
    //println!("{}", r3);
    //println!("{}, {}", r1, r2);

    //let r1 = dangle();
    //let r2 = r1;
    //println!("{} {}", r1,);
    //println!("{}", r1);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    first_word(&s);
}


fn dangle() -> &'static str { // dangle returns a reference to a String

    let s = "hello"; // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!


  fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    println!("{:#?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}