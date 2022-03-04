use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    println!("Hello, world!");

    let file_name = "hello.txt";
    //open_file(file_name);
    let data = read_data_from_file(file_name);

    match data {
        Ok(d) => println!("{:?}", d),
        Err(error) => println!("{:?}", error),
    };
    //let f = File::open(file_name).unwrap();
    //let f = File::open(file_name).expect("Faild to open file");

    //open_file(file_name);
    //panic!("crash and burn");
}

fn open_file(file_name: &str) {
    let f = File::open(file_name);

    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_err => {
                panic!("Problem opening the file {:?}", other_err)
            }
        },
    };
}

fn read_data_from_file(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}
