fn main()
{
    println!("hey!");
}
#[test]
fn ass(){
  let x = 4;
  let eq_x = |z| z == x;
  let y = 4;
  assert!(eq_x(y));
}
