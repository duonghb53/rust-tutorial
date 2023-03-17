fn main() {
    let x = 5;
    let y = Some(-111);

    let sum = x + y.unwrap_or(1);

    println!("sum = {}", sum);

    decimals(Coin::Solana);
    decimals(Coin::Bitcoin(Balance::Shark));

    let five = Some(5);
    let six = plus_one(five);
    println!("six = {:#?}", six);

    let _none = plus_one(None);
    println!("None = {:#?}", _none);
}

// Match Option

#[derive(Debug)]
enum Balance {
    Small,
    Intermediate,
    Fish,
    Shark,
}

enum Coin {
    Solana,
    Ethereum,
    Near,
    Bitcoin(Balance),
}

fn decimals(coin: Coin) -> u8 {
    match coin {
        Coin::Solana => {
            println!("Solana Match");
            1
        }
        Coin::Ethereum => 10,
        Coin::Near => 20,
        Coin::Bitcoin(balance) => {
            println!("I am a {:#?}", balance);
            100
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 1),
        _ => None,
    }
}
