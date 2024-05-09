use geo::{coord, point, Intersects, Line, LinesIter, Point, Rect, Translate};

#[derive(Debug)]
pub struct Config {
    pub ball: Point,
    pub angle: f64,
}

#[derive(Debug, PartialEq)]
pub struct Collision {
    pub line1: Line,
    pub line2: Line,
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let table: Rect = Rect::new(coord! {x:-90.,y:-45.}, coord! {x:90.,y:45.});

    let collision = get_collision(&config, &table);

    dbg!(collision);

    Ok(())
}

pub fn get_collision(config: &Config, table: &Rect) -> Option<Collision> {
    let ball_line: Line;
    if config.angle == 90. {
        ball_line = Line::new(config.ball, point!(x:config.ball.x(), y:45.))
    } else if config.angle == -90. {
        ball_line = Line::new(config.ball, point!(x:config.ball.x(), y:-45.))
    } else {
        let angle_tan = config.angle.to_radians().tan();
        let ball_destination = config.ball.translate(180., angle_tan * 180.);
        ball_line = Line::new(config.ball, ball_destination);
    }

    dbg!(ball_line, table);

    let intersect_line = table.lines_iter().find(|line| line.intersects(&ball_line));

    match intersect_line {
        Some(intersect_line) => {
            return Some(Collision {
                line1: ball_line,
                line2: intersect_line,
            })
        }
        None => return None,
    }
}
