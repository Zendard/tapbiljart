use geo::{
    coord, line_intersection::*, point, Intersects, Line, LinesIter, Point, Rect, Rotate, Translate,
};
use libm::atan;

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

    let collision = get_collision(&config, &table).expect("No collision");
    let bounce_line = get_bounce_line(&collision);
    dbg!(bounce_line);

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

pub fn get_bounce_line(collision: &Collision) -> Line {
    // if collision.line1.slope() == f64::INFINITY {
    //     return Line::new(
    //         collision.line1.end_point(),
    //         collision.line1.end_point().translate(-180., 0.),
    //     );
    // }
    // let slope = collision.line2.slope();
    //
    // if slope == 0. {
    //     Line::new(
    //         collision.line1.end_point(),
    //         collision
    //             .line1
    //             .end_point()
    //             .translate(180., collision.line1.slope() * -180.),
    //     )
    // } else {
    //     Line::new(
    //         collision.line1.end_point(),
    //         collision
    //             .line1
    //             .end_point()
    //             .translate(collision.line1.slope() * -90., 90.),
    //     )
    // }

    let intersect =
        match line_intersection(collision.line1, collision.line2).expect("Lines don't collide") {
            LineIntersection::SinglePoint { intersection, .. } => intersection,
            LineIntersection::Collinear { intersection } => {
                panic!("Collision lines are colinear in {:#?}", intersection)
            }
        };
    let rotate_degrees = 2. * atan(collision.line1.slope()).to_degrees();

    let rotated_line = collision
        .line1
        .rotate_around_point(-rotate_degrees, intersect.into());

    Line {
        start: rotated_line.end_point().into(),
        end: rotated_line.start_point().into(),
    }
}
