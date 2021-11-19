fn main() {
    // Tạo 1 biến x = 5
    let x = 5;

    // Tạo biến x, gán giá trị y = x = 5
    let y = x;
    println!("x: {}, y: {}", x, y); // lúc này vẫn có thể in ra x, y vì x,y là kiểu giá trị int, fix size được lưu trong stack
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
        println!("s: {}", s);
    }

    // Tạo ra biến 1 giá trị = hello cung cấp 1 chuỗi tham chiếu và cấp phát bộ nhớ trên Heap
    let s1 = String::from("hello");
    let s2 = s1;

    //println!("s: {}", s1); // Lỗi ở đây vì biến 1 đã được biến 2 borrow, giờ giá trị "hello" thuộc owner của s2 và s1 đã dược giải phóng bộ nhớ
    println!("s: {}", s2);

    // Nhưng tại sao dòng số 6 cũng như thế nhưng không bị lỗi, là do dòng số 6 thực hiện Copy y sẽ có 1 địa chỉ stack khác chứ không dùng heap.
    // Các kiểu dữ liệu nguyên thuỷ: integer, boolean, float, char sẽ dùng bộ nhớ stack và Copy chứ không phải borrow như Heap.

    let s = String::from("some_string"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
                        //println!("{}", s); // Nếu sử dụng ở đây thì sẽ lỗi vì biến s đã dùng trong hàm takes_ownership và được giải phóng ở đó
    let x = 100; // x comes into scope
    makes_copy(x);
    println!("{}", x); // biến X không lỗi vì biến x là kiểu nguyên thuỷ


    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
    //println!("{}", s1); 

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
