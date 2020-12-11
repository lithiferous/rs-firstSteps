use std::fmt::Display;

struct Masterpiece<'a>{
    part: &'a str,
}

impl<'a> Masterpiece<'a> {
    fn return_part_announced(&self, annouce: &str) -> &str
    {
        println!("Hey, my dickerino is huge: {}", annouce);
        self.part
    }
}

fn longest_announced<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where T: Display
{
    println!("Ducks go on the hunt: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main()
{
    let _s: &'static str = "I live forever";
    let novel = String::from("My name is Alex. I like playin with Rust...");
    let first_sentence = novel.split(".")
        .next()
        .expect("Could not find a dot - '.'");

    let i = Masterpiece{part:first_sentence};
    println!("Hello, partial oeuvre {}", i.return_part_announced("Follow my lead!"));
    println!("annotation her sucks: {}", longest_announced("abcd", "xyz", "Eat my tiger!"));
}
