#[derive(Debug)]
struct Box {
    hrz: u16,
    vrt: u16,
}

impl Box {
    fn area(&self) -> u16
    {
        self.hrz * self.vrt
    }
    fn holds(&self, b: &Box) -> bool{
        self.hrz >= b.hrz &&
        self.vrt >= b.vrt
    }
    fn sqr(size: u16) -> Box {
        Box{hrz:size, vrt:size}
    }
}

fn main()
{
    let rec = Box{hrz:16, vrt:12};
    let rec1 = Box{hrz:8, vrt:12};
    let rec2 = Box{hrz:32, vrt:50};
println!("Total area of first - {}", rec.holds(&rec1));
println!("Total area of first - {}", rec.holds(&rec2));
println!("This is an ass of 4: {:?}", Box::sqr(4));

}
