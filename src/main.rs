use geo::point;
use std::io;
use tapbiljart::Config;

fn main() {
    let config = get_config();
    dbg!(&config);

    if let Err(e) = tapbiljart::run(config) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}

fn get_config() -> Config {
    let mut config = Config {
        ball: point! { x: 0., y: 0. },
        angle: 0.,
    };
    let mut buffer = String::new();

    println!("Ball x coordinate (0): ");
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.pop();
    *config.ball.x_mut() = buffer.parse().unwrap_or_default();
    buffer = String::new();

    println!("Ball y coordinate (0): ");
    io::stdin().read_line(&mut buffer).unwrap_or(0);
    buffer.pop();
    *config.ball.y_mut() = buffer.parse().unwrap_or_default();
    buffer = String::new();

    println!("Ball shoot angle (0): ");
    io::stdin().read_line(&mut buffer).unwrap_or(0);
    buffer.pop();
    config.angle = buffer.parse().unwrap_or_default();

    config
}
