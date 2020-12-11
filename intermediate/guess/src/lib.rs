pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess
    {
        if value < 1 || value > 100 {
            panic!("Guess should be within boundaries [1...100], got {}", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
    #[test]
    #[should_panic]
    fn smaller_than_1() {
        Guess::new(-200);
    }

}
