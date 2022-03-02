fn main() {
    println!("Hello, world!");
    let config_max = Some(3);
    if let Some(max) = config_max {
        println!("Maximum is {:?}", max);
    }
}
