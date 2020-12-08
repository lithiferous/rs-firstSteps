#[derive(Debug)]
enum IpAddrType{
    V4(u8,u8,u8,u8),
    V6(String),
}

fn main()
{
    let v4 = IpAddrType::V4(127,0,0,1);
    let v6 = IpAddrType::V6(String::from("::1"));

    print(v4);
    print(v6);

    let x: i8 = 5;
    let y: Option<i8> = Some(4);
    println!("{}", x * y.unwrap());

}

fn print(ip: IpAddrType){
println!("Hello,- {:?}", ip);
}
