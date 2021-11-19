#[warn(dead_code)]
const PI: f32 = 3.14;

fn main() {
    mut_variable();
    shadowing();
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("Icon: {}", heart_eyed_cat);

    let array = [1, 1, 1];
    println!("The value of array is: {:?}", array);
}

fn mut_variable() {
    // Khai báo biến x = 5 kiểu immutable: không thể thay đổi
    let mut x = 5;
    println!("The value of x is: {}", x);

    // Nếu thay đổi giá trị biến ở đây thì sẽ lỗi
    x = 6;
    println!("The value of x is: {}", x);
}

fn shadowing() {
    // Khai báo biến x = 5
    let x = 5;

    // Shadowing tạo 1 biến x mới dung giá trị của x cũ
    let x = x + 1;

    {
        // biến x này trong Scope thì chỉ tồn tại trong scope này
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
