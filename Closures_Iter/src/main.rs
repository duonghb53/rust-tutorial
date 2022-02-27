fn main() {
    //println!("Hello, world!");

    let data: Vec<u8> = vec![1, 2, 3, 4, 5];
    let data1: Vec<u8> = data
        .iter()
        .map(|num| num * 3)
        .filter(|num| num > &10)
        .collect();

    println!("{:?}", data1);
}

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    //unimplemented!("implement `fn evens`");
    // TODO: remove this; it's only necessary to allow this function to compile
    // before the student has done any work.
    iter.enumerate().filter(|(i, e)| i % 2 == 0).map(|(i, e)| e)
}

pub struct Position {
    x: i16,
    y: i16,
}
impl Position {
    pub fn manhattan(&self) -> i16 {
        self.x.abs() + self.y.abs()
    }
}
