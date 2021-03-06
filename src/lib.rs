use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args)->Result<Config, &'static str> {
        if args.len()<3 {
            return Err("引数が足りないすよ");
            //panic!("引数が足りないす");
        }

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("検索文字列を指定してよ"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("ファイル名を指定してよ"),
        };

        let case_sensitive = match env::var("CASE_INSENSITIVE") {
            Ok(one)=>{
                if one == "1" {
                    false
                } else {
                    true
                }
            },
            _ => {
                true
            }
        };

        println!("case_:{}",case_sensitive);

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config:Config) -> Result<(),Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        println!("search()");
        search(&config.query, &contents)
    } else {
        println!("search_case_insensitive()");
        search_case_insensitive(&config.query,&contents)
    };
    
    for line in results {
        println!("{}",line);
    }
/*
    println!("ファイルの中身 --（ココから）--");
    println!("{}",contents);
    println!("--（ココまで）--");
*/
    Ok(())
}

fn search<'a>(query:&str,contents:&'a str)->Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query:&str,contents:&'a str)->Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "鼻たれ"       ;
        let contents = "\
人間五十年
下天のうちをくらぶれば
夢幻のごとくなり
鼻たれ小僧
夢を忘れるな";

        assert_eq!(
            vec!["鼻たれ小僧"],
            search(query,contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "abc";
        let contents = "\
xyz
abcd
AbCDXYz";
        assert_eq!(
            vec!["abcd"],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "abc";
        let contents = "\
xyz
abcd
AbCDXYz";
        assert_eq!(
            vec!["abcd","AbCDXYz"],
            search_case_insensitive(query, contents)
        );

    }
}

