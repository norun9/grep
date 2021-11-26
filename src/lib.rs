use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough argumanets");
        }
        let query;
        let filename;
        if args.len() < 2 {
            query = args[0].to_string();
            filename = args[1].to_string();
        }
        Ok(Config { query, filename })
    }

    fn get_info<'a>(args: &'a [String]) -> (&'a str, &'a str) {
        let q = args[0].to_string();
        let f = args[1].to_string();

        (&q, &f)
    }
}

// 'a: https://laysakura.github.io/2020/05/21/rust-static-lifetime-and-static-bounds/
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
