fn main() {
    let mut s = String::from("Hello");
    let word = first_word(&s);
    s.push_str(word);

    println!("{}", s);
}

fn first_word(s: &str) -> &str
{
    let bytes = s.as_bytes();

    for (i, &it) in bytes.iter().enumerate(){
        if it == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}

