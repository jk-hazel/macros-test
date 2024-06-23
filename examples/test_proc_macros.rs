use anyhow::Result;
use macros::EnumFrom;

#[derive(Debug, EnumFrom)]
#[allow(dead_code)]
enum Direction {
    Up(DirectionUp),
    Down(i32),
    Left(String),
    Right(f64),
}

#[derive(Debug)]
#[allow(dead_code)]
struct DirectionUp {
    spench: i32,
}

// impl From<DirectionUp> for Direction {
//     fn from(value: DirectionUp) -> Self {
//         Direction::UP(value)
//     }
// }
impl DirectionUp {
    pub fn new() -> Self {
        Self { spench: 32 }
    }
}

#[allow(dead_code)]
impl Direction {
    pub fn new() -> Self {
        Self::Up(DirectionUp::new())
    }
}

fn main() -> Result<()> {
    let up: Direction = DirectionUp::new().into();
    let down: Direction = 32.into();
    let left: Direction = "left".to_string().into();
    let right: Direction = 3.14.into();
    println!(
        "up:{:?},douwn:{:?},left:{:?},right:{:?}",
        up, down, left, right
    );
    Ok(())
}
