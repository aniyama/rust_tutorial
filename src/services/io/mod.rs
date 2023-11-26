use crate::models;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_date_or_create_date(path: &str) -> Vec<models::Item> {
    let file = File::open(path);
    match file {
        Ok(f) => {
            let buf_reader = BufReader::new(f);
            serde_json::from_reader(buf_reader).expect("デシリアライズに失敗")
        }
        Err(_) => {
            println!("新規ファイルを作成しまう");
            Vec::new()
        }
    }
}

pub fn read_data_or_panic(path: &str) -> Vec<models::Item> {
    let file = File::open(path).expect("ファイルがオープンでき魔s寝でした");
    let buf_reader = BufReader::new(file);
    let data: Vec<_> = serde_json::from_reader(buf_reader).expect("デシリアライズに失敗");

    if data.len() == 0 {
        panic!("not data");
    }
    data
}

pub fn write_to_json(data: &Vec<models::Item>, path: &str) {
    let json_data = serde_json::to_string_pretty(data).expect("Jsonへのシリアライズ失敗");
    let mut f = File::create(path).expect("書き込みファイルのオープンに失敗愛sました");
    writeln!(f, "{}", json_data).expect("ファイルへの書き込みに失敗");
    println!("項目登録完了");
}
