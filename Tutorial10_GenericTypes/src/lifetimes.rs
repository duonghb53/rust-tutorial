pub fn lifetime() {
    let r;
    let x = 5;
    r = &x;

    println!("r: {}", r);
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}