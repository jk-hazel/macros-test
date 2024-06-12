use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    // let err = f3(f2(f1("test-err")?)?)?;
    // println!("{}", err);
    let err = test_try!(f3(test_try!(f2(test_try!(f1("test-err"))))));
    println!("{}", err);
    Ok(())
}

fn f1(s: impl AsRef<str>) -> Result<String> {
    Ok(format!("f1:{}", s.as_ref()))
}

fn f2(s: impl AsRef<str>) -> Result<String> {
    Ok(format!("f2:{}", s.as_ref()))
}

fn f3(s: impl AsRef<str>) -> Result<String> {
    Err(anyhow!("f3:{}", s.as_ref()))
}

// ? operator how to simulate it
#[macro_export]
macro_rules! test_try {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err.into()),
        }
    };
}
