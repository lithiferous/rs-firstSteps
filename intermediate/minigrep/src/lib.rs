use std::{env, fs};
use std::error::Error;

pub struct Cfg {
    pub query: String,
    pub file:  String,
    pub case:  bool,
}

impl Cfg {
    pub fn new(args: &[String]) -> Result<Cfg, &'static str>
    {
        if args.len() < 3 {
            return Err("wrong number of arguments");
        }
        let file = args[1].clone();
        let query = args[2].clone();
        let case = env::var("CASE").is_err();

        Ok(Cfg{file, query, case})
    }
}

pub fn run(cfg: Cfg) -> Result<(), Box<dyn Error>>
{
    let text = fs::read_to_string(cfg.file)?;

    let res = if cfg.case {
        search(&cfg.query, &text)
    } else {
        search_case_ns(&cfg.query, &text)
    };

    for ln in res {
        println!("{}", ln);
    }
    Ok(())
}

pub fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str>
{
    let mut res = Vec::new();
    for ln in text.lines() {
        if ln.contains(query){
            res.push(ln);
        }
    }

    res
}

pub fn search_case_ns<'a>(query: &str, text: &'a str) -> Vec<&'a str>
{
    let mut res = Vec::new();
    let query = query.to_lowercase();
    for ln in text.lines() {
        if ln.to_lowercase().contains(&query){
            res.push(ln);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive()
    {
        let query = "duct";
        let text = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, text)
        );
    }

    #[test]
    fn case_insensitive()
    {
        let query = "rUsT";
        let text = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_ns(query, text)
        );
    }
}
