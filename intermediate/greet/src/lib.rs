pub fn greet(name: &str) -> String
{
    format!("Hello you dumb {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_contains_name()
    {
        let res = greet("Ass");
        assert!(res.contains("Ass"));
    }
}
