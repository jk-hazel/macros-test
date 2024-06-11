use anyhow::Result;
use macros::test_vec;

fn main() -> Result<()> {
    let mut v: Vec<i32> = test_vec!["1".parse()?, "2".parse()?, "3".parse()?];
    v.push(3);
    v.push(4);
    println!("{:?}", v);
    Ok(())
}
