#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String>
    {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Not in our universe!"))
        }
    }
}
