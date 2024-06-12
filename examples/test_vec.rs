use anyhow::Result;

fn main() -> Result<()> {
    let mut v: Vec<i32> = test_vec!["1".parse()?, "2".parse()?, "3".parse()?];
    v.push(3);
    v.push(4);
    println!("{:?}", v);
    Ok(())
}

// test_vec! = test_vec!{1,2,3};//Vec<i32>
#[macro_export]
macro_rules! test_vec {
    () => {
        Vec::new()
    };

    ($elem:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    };

    ($($x:expr),+ $(,)?) => {{
        <[_]>::into_vec(Box::new([$($x), *]))
    }};

}
