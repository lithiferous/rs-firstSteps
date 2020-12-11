use std::cmp::PartialOrd as PO;

fn largest<T>(list: &[T]) -> T
    where T: PO + Copy
{
    let mut lgt = list[0];
    for &it in list.iter(){
        if it > lgt{
            lgt=it
        }
    }
    lgt
}

fn main()
{
    let num_list = vec![1,2,3,4,20,54,99];
    let res = largest(&num_list);
    println!("Largest number is: {}", res);

    let c_list = vec!['c','a','3','d','q'];
    let res = largest(&c_list);
    println!("Largest letter is: {}", res);

}
