use std::io;

use tapbiljart::Config;

fn main() {
    let mut config = Config {
        ball: tapbiljart::Point { x: 0.0, y: 0.0 },
    };
    let mut buffer = String::new();

    println!("Ball x coordinate (0): ");
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.pop();
    config.ball.x = buffer.parse().unwrap_or_default();

    println!("Ball y coordinate (0): ");
    io::stdin().read_line(&mut buffer).unwrap_or(0);
    buffer.pop();
    config.ball.y = buffer.parse().unwrap_or_default();

    dbg!(config);

    if let Err(e) = tapbiljart::run() {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}
