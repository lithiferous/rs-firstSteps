#[derive(Debug)]
struct Masterpiece <'a>{
    part: &'a str,
}

fn main()
{
    let novel = String::from("My name is Alex. I like playin with Rust...");
    let first_sentence = novel.split(".")
        .next()
        .expect("Could not find a dot - '.'");

    let i = Masterpiece{part:first_sentence};
    println!("Hello, partial oeuvre {:?}", i);
}
