use anyhow::Result;
use macros::EnumFrom;

#[derive(Debug, EnumFrom)]
#[allow(dead_code)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down(i32),
    Left(String),
    Right(f64),
}

#[derive(Debug)]
#[allow(dead_code)]
struct DirectionUp<T> {
    spench: T,
}

// impl From<DirectionUp> for Direction {
//     fn from(value: DirectionUp) -> Self {
//         Direction::UP(value)
//     }
// }
impl<T> DirectionUp<T> {
    pub fn new(t: T) -> Self {
        Self { spench: t }
    }
}

#[allow(dead_code)]
impl<T> Direction<T> {
    pub fn new(t: T) -> Self {
        Self::Up(DirectionUp::new(t))
    }
}

fn main() -> Result<()> {
    let up: Direction<i32> = DirectionUp::new(32).into();
    let down: Direction<i32> = 32.into();
    let left: Direction<i32> = "left".to_string().into();
    let right: Direction<i32> = 3.14.into();
    println!(
        "up:{:?},douwn:{:?},left:{:?},right:{:?}",
        up, down, left, right
    );
    Ok(())
}
