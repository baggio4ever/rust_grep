extern crate rust_grep;

use std::env;
use std::process;
use rust_grep::Config;

fn main() {
    //println!("Hello, rust_grep!");

    let args:Vec<String> = env::args().collect();
    println!("args: {:?}",args);

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("引数の解析で問題アリ：{}",err);
        process::exit(1);
    });

    if let Err(e) = rust_grep::run(config) {
        println!("エラーでございます：{}",e);
        process::exit(1);
    }
}