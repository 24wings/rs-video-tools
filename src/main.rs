#![feature(path_try_exists)]
extern crate serde;
extern crate serde_derive;
use serde_derive::*;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub mov_dir: String,
    pub wav_dir: String,

    pub dist_dr: String,
}

fn main() {
    println!("Hello, world!");
    println!("{}", "hello");
    let mut config_file_path = PathBuf::new();
    let mut current_dir = std::env::current_dir().unwrap().clone();
    config_file_path.push(current_dir);
    config_file_path.push("config.toml");
    let is_not_exist_config = std::fs::try_exists(&config_file_path).is_err();

    if is_not_exist_config {
        let err_msg = format!("配置文件:{:?}不存在", config_file_path.to_str().unwrap());
        panic!("{}", &err_msg);
    } else {
        let content = std::fs::read_to_string(config_file_path).unwrap();
        let config: Config = toml::from_str(&content).expect("配置文件格式错误");
        println!("{:#?}", config);
    }

    let a = 1;
    // toml::toml! {};
}
