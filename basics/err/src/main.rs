use std::fs::{self, File};
use std::io::{self, Read};

fn main()
{
    let fname = "help.txt";
    let res   = read_word_from_file(&fname);
    println!("{:?}", res.unwrap());
    let res   = read_word_from_file2(&fname);
    println!("{:?}", res.unwrap());
    let res   = read_word_from_file3(&fname);
    println!("{:?}", res.unwrap());
    let res   = fin(&fname);
    println!("{:?}", res.unwrap());
}

fn read_word_from_file(fname: &str) -> Result<String, io::Error>
{
    let f = File::open(fname);

    let mut f = match f {
        Ok(fl) => fl,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_word_from_file2(fname: &str) -> Result<String, io::Error>
{
    let mut f = File::open(fname)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_word_from_file3(fname: &str) -> Result<String, io::Error>
{
    let mut s = String::new();
    File::open(fname)?.read_to_string(&mut s)?;
    Ok(s)
}

fn fin(fname: &str) -> Result<String, io::Error>
{
    fs::read_to_string(fname)
}
