extern crate rust_grep;

use std::env;
use std::process;
use rust_grep::Config;

fn main() {
    //println!("Hello, rust_grep!");

    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("引数の解析で問題ありました： {}",err);
        process::exit(1);
    });

    if let Err(e) = rust_grep::run(config) {
        eprintln!("エラーなのよ： {}",e);
        process::exit(1);
    }
}
