const PI: f32 = 3.14;

fn main() {
    mut_variable();
    shadowing();
}


fn mut_variable(){
     // Khai báo biến x = 5 kiểu immutable: không thể thay đổi
     let mut x = 5;
     println!("The value of x is: {}", x);
 
     // Nếu thay đổi giá trị biến ở đây thì sẽ lỗi
     x = 6;
     println!("The value of x is: {}", x);
}

fn shadowing(){
    // Khai báo biến x = 5
    let x = 5;


    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}