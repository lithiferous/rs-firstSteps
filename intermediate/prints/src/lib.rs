fn prints_and_returns(a: i32) -> i32
{
    println!("I got value of {}", a);
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_pass() {
        let val = prints_and_returns(10);
        assert_eq!(10, val);
    }
    #[test]
    #[ignore]
    fn this_fail() {
        let val = prints_and_returns(8);
        assert_eq!(10, val);
    }
}
