use adder;

mod common;

#[test]
fn it_adds_two()
{
    common::setup();
    assert_eq!(2+2, adder::add_two(2))
}
