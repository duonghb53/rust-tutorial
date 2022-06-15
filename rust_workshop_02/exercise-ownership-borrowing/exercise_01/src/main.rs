//Exercise 1
// Mục đích: giải quyết vấn đề ownership and borrowing không dùng clone()
fn main() {
    let x = change_value(1,&mut 20);
    println!("{}",x);
}

fn change_value(input:u32, output: &mut u32) -> u32{
    if input == 1 {
        *output = 3;
    }
    else {
        *output = 4;
    }
    *output
}