#[derive(Debug)]
pub struct Config {
    pub ball: Point,
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
