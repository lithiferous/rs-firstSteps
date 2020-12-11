use std::format;
use std::io::{self, Write};

struct User {
    name:   String,
    email:  String,
    age:    u8,
    active: bool,
}

fn main() {
println!("Hello, world!");
    let user = User {
        name: String::from("bane"),
        email: String::from("alex.degt.v@gmail.com"),
        age: 24,
        active: true,
    };
println!("Hi there: {}", user.name);

    let mut name = inp_get(&"name".to_string());
    let mut email = inp_get(&"email".to_string());

    let new_user = user_get(name, email);

println!("Build an old man with name {} email {}", new_user.name,
                                                   new_user.email);

    struct Car(String, u8, u8);

    let volvo = Car("XC90".to_string(), 12, 4);
    println!("We have a car with model {} aged {} with {} wheels", volvo.0,
                                                                   volvo.1,
                                                                   volvo.2);
}

fn user_get(name: String,
            email: String) -> User
{
    User {
        name,
        email,
        age: 90,
        active: true
    }
}

fn inp_get(inp: &String) -> String
{
print!("Please, specify your {}: ", inp);
    io::stdout().flush().unwrap();

    let mut x = String::new();
    io::stdin().read_line(&mut x);
    x
}
