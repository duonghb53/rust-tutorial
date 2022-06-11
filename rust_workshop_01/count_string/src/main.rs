use std::io;
use std::io::Read;
use std::fs::File;
use std::env;
use regex::bytes::Regex;
const FILE_NAME: &str = "./src/data.txt";

#[derive(Debug)]
enum ModeSearch {
    LowerUpperCase,
    Normal,
}

fn main() {
    loop {
        let exit = input_string("Start find count string. \nEnter anything to start. \nPlease enter '-1' to exit");

        let mode = select_mode();
        let input = input_string("Please input string: ");
        let data = read_data_from_file(FILE_NAME);

        let count_string = count_string(&mode, input, data);
        println!("Số kí tự tìm được ở mode: {:?} là {}", mode, count_string);

        if exit == "-1" {
            break;
        }
    }

}

fn input_string(title: &str) -> String {
    let mut input = String::new();
    println!("{}", title);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.pop();
    input
}

fn read_data_from_file(file_name:&str) -> String {
    let mut file = File::open(file_name).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    contents
}

fn utf8_to_string(bytes: Vec<u8>) -> String {
    let s = String::from_utf8(bytes).expect("Found invalid UTF-8");
    s
}

fn select_mode() -> ModeSearch {
    let mode = input_string("Please select mode: \n1. Lower and upper case\n2. Normal");
    match mode.trim() {
        "1" => ModeSearch::LowerUpperCase,
        "2" => ModeSearch::Normal,
        _ => panic!("Invalid mode"),
    }
}

fn count_string(mode: &ModeSearch, input: String, data: String) -> usize {
    let count = match mode {
        ModeSearch::LowerUpperCase => data.matches(&input).count(),
        ModeSearch::Normal => {
            let data = data.to_lowercase();
            let input = input.to_lowercase();
            data.matches(&input).count()
        },
    };

    count
}