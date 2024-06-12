use std::fmt::format;

use anyhow::{anyhow, Result}



fn main() -> Result<()> {
    let err = f3(f2(f1("测试")?)?);
    print!("{:?}",err);
    Ok(())
}

fn f1 (s: impl AsRef<str>) -> Result<String> {
    Ok(format!("f1:{}", s.as_ref()))
}


fn f2 (s: impl AsRef<str>) -> Result<String>{
    Ok(format!("f2:{}", s.as_ref()))
}

fn f3(s: impl AsRef<str>) -> Result<String> {
    Err(anyhow!("f3:{}", s.as_ref()))
}