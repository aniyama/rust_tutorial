use rust_tutorial::services;
use std::io;

fn main() {
    let mut input_type = String::new();

    println!("0: 登録、1: 集計");
    io::stdin().read_line(&mut input_type).unwrap();
    let input_type: u8 = input_type.trim().parse().expect("数値で入力してください");

    services::validate::InputValidator::validate_input_type(input_type);
    if input_type == 0 {
        services::register::run("store/data.json");
    } else if input_type == 1 {
        services::summarize::run("store/data.json");
    }
}
