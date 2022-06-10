extern crate string_builder;
use string_builder::Builder;

fn main() {
    let s = String::with_capacity(1024);
    let mut s = String::new();
    let len = s.len();
    let capacity = s.capacity();
    let ptr = s.as_ptr();
    println!("Ptr: {:?}, Len: {}, Capacity: {}", ptr, len, capacity);

    s = s + (" Hello");
    let len = s.len();
    let capacity = s.capacity();
    let ptr = s.as_ptr();
    println!("Ptr: {:?}, Len: {}, Capacity: {}", ptr, len, capacity);

    s = s + (" World!!!!");
    let len = s.len();
    let capacity = s.capacity();
    println!("Ptr: {:?}, Len: {}, Capacity: {}", ptr, len, capacity);

    s = s + (" 213131");
    let len = s.len();
    let capacity = s.capacity();
    let ptr = s.as_ptr();
    println!("Ptr: {:?}, Len: {}, Capacity: {}", ptr, len, capacity);

    s.push_str(" World!!!!");
    let len = s.len();
    let capacity = s.capacity();
    let ptr = s.as_ptr();
    println!("Ptr: {:?}, Len: {}, Capacity: {}", ptr, len, capacity);

    let mut b = Builder::default();
    b.append("it");
    b.append(' ');
    b.append("works!");

    let len = b.len();
    println!("Len: {}", len);

    let mut b = Builder::new(200);
    b.append("In summary, use String if you need owned string data \n
    (like passing strings to other threads, or building them at runtime), \n
    and use &str if you only need a view of a string.");
    let ptr = s.as_ptr();
    println!("Ptr: {:?}, Len: {}, Capacity: {}", ptr, len, capacity);
    println!("Len: {}", len);

    b.append("In summary, use String if you need owned string data 
    (like passing strings to other threads, or building them at runtime), 
    and use &str if you only need a view of a string.");
    let len = b.len();
    println!("Len: {}", len);

    let mut s = String::with_capacity(300);
    let len = s.len();
    let capacity = s.capacity();
    let ptr = s.as_ptr();
    println!("Ptr: {:?}, Len: {}, Capacity: {}", ptr, len, capacity);

    s.push_str("In summary, use String if you need owned string data \n
    (like passing strings to other threads, or building them at runtime), \n
    and use &str if you only need a view of a string.");
    let len = s.len();
    let capacity = s.capacity();
    let ptr = s.as_ptr();
    println!("Ptr: {:?}, Len: {}, Capacity: {}", ptr, len, capacity);

    s.push_str("In summary, use String if you need owned string data 
    (like passing strings to other threads, or building them at runtime), 
    and use &str if you only need a view of a string.");
    let len = s.len();
    let capacity = s.capacity();
    let ptr = s.as_ptr();
    println!("Ptr: {:?}, Len: {}, Capacity: {}", ptr, len, capacity);

}
