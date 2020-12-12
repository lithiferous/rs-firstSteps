use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main()
{

    fn generate_world(intens: u32, seed: u32) {
        let mut res = Cacher::new(|_num| {
            println!("Computing slowly!");
            thread::sleep(Duration::from_secs(2));
            intens
        });
        if intens < 25 {
            println!("I have generated {} chunks", res.value(intens));
        } else {
            if seed == 3 {
                println!("Welcome dev");
            }  else {
                println!("I have generated {} NPCs", res.value(intens));
            }
        }
    };

    let user_spec = 20;
    let seed = 4;
    generate_world(
        user_spec,
        seed
    );
}

struct Cacher<T>
where T: Fn(u32) -> u32
{
    calculation: T,
    dict: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T>
    {
        Cacher {
            calculation,
            dict: HashMap::new()
        }
    }

    fn value(&mut self, arg: u32) -> u32
    {
            match self.dict.get(&arg){
                Some(v) => v.to_owned(),
                None    => {
                    let v = (self.calculation)(arg);
                    self.dict.entry(v).or_insert(arg);
                    v
                },
            }
    }
}

#[test]
fn call_fn() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v2, 2);
}
