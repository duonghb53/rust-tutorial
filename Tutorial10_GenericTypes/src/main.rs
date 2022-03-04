pub mod aggregator;

pub mod lifetimes;

use aggregator::{Summary, Tweet};
use std::cmp::PartialOrd;

fn main() {
    println!("Hello, world!");
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 10.2, y: 20.5 };

    let p = Point { x: 10, y: 20 };
    println!("p.x = {}", p.x());

    let p1 = Point { x: 10, y: 20.2 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("DuongHB"),
        content: String::from("Xin chao moi nguoi"),
        reply: false,
        retweet: false,
    };

    println!("New tweet: {}", tweet.summarize());

    notity(&tweet);
}

pub fn notity<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<T2, U2>(self, other: Point<T2, U2>) -> Point<T, U2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
