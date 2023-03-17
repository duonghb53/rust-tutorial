/*
    Nội dung game là viết 1 chương trình nhập vào 1 số từ bàn phím,
    xử lý input đó xem có đúng mong đợi hay không
    Mình sẽ viết 1 hàm tạo 1 số random -> nhập số từ bàn phím, 
    kiểm tra xem số nhập vào <, >, = số random. Nếu = thì thoát chương trình
*/

// Bỏ qua warning cách đặt tên folder, file, fn....
#![allow(non_snake_case)]
// Thư viện chuẩn in/out cho phép nhập kí tự từ bàn phím
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
    guessing_game();
}

// Hàm nhập vào kí tự từ bàn phím
fn guessing_game() {
    println!("Guess the number!");

    // Random 1 số từ 1->100 và in ra
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess:");
        
        // Tạo 1 biến guest kiểu dữ liệu String để lưu giá trị nhập vào từ bàn phím
        // Kiểu mut: mutable biến có thể thay đổi sẽ được học rõ hơn ở Tutorial 3
        // String::new() là tạo 1 chuỗi rỗng
        let mut guess = String::new();

        // Nhận kí tự từ bàn phím và gán vào biến Guess
        // read_line sẽ lấy tất cả các kí tự bạn nhập từ bàn phím vào đến khi nhấn Enter
        // &mut là kiểu tham chiếu cho phép bạn không cần tạo ra bản sao mới của dữ liệu
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // tạo 1 biến guess convert từ string -> u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
