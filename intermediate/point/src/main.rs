use std::fmt::Display;

// P -> Pair
struct P<T> {
    x: T,
    y: T,
}

impl <T> P<T>
{
    fn new(x: T, y: T) -> Self
    {
        Self {
            x,
            y,
        }
    }
}

impl <T> P<T>
    where T: Display + PartialOrd
{
    fn cmp_disp(&self)
    {
        if self.x >= self.y {
println!("The bigger property is x with value of {}", self.x);
        } else {
println!("The bigger property is y with value of {}", self.y);
        }
    }
}

fn main()
{
    let p = P::new(1,4);
    p.cmp_disp();
    println!("Hello, world!");
}
