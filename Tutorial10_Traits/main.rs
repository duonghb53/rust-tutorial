fn main(){
    let player_1 = Player {
        first_name: String::from("Dưỡng"),
        last_name: String::from("Hà")
    };

    println!("Full name: {}", player_1.full_name());

    let player_2 = Player {
        first_name: "Roger".to_string(),
        last_name: "Federer".to_string(),
    };

    println!("{}", player_2.full_name());
}

struct Player {
    first_name: String,
    last_name: String
}

// Ví dụ về sử dụng implement trực tiếp trong struct với hàm mặc định
// Nếu impl ở đây thì Struct sẽ mặc định gọi đến hàm này
impl Player {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

// Chỉ khai báo hàm với hàm mặc định. Nếu không impl thì nó sẽ gọi ra hàm này
trait FullName {
    fn full_name(&self) -> String {
        String::from("No string...")
    }
}


// Khi khai báo trait FullName cho Player thì mình cần implement toàn bộ các hàm trong FullName
// Các hàm này sẽ lấy gái trị self là của Player
impl FullName for Player {
    fn full_name(&self) -> String {
        format!("My name is: {} {}", self.first_name, self.last_name)
    }
}