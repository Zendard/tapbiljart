#[cfg(test)]
mod tests {
    use geo::{coord, line_intersection::line_intersection, point, Line, Rect};
    use tapbiljart::{Collision, Config};

    #[test]
    fn collision_90() {
        let table: Rect = Rect::new(coord! {x:-90.,y:-45.}, coord! {x:90.,y:45.});

        let config = Config {
            ball: point!(x:0.,y:0.),
            angle: 90.,
        };

        let collision = tapbiljart::get_collision(&config, &table).unwrap();
        assert_eq!(
            collision,
            Collision {
                line1: Line::new(config.ball, point!(x:0.,y:45.)),
                line2: Line::new(point!(x:-90.,y:45.), point!(x:90.,y:45.))
            }
        )
    }

    #[test]
    fn collision_min_90() {
        let table: Rect = Rect::new(coord! {x:-90.,y:-45.}, coord! {x:90.,y:45.});

        let config = Config {
            ball: point!(x:0.,y:0.),
            angle: -90.,
        };

        let collision = tapbiljart::get_collision(&config, &table).unwrap();

        assert!(is_colinear(
            &collision.line1,
            &Line::new(config.ball, point!(x:0.,y:-45.)),
        ));
        assert!(is_colinear(
            &collision.line2,
            &Line::new(point!(x:90.,y:-45.), point!(x:-90.,y:-45.))
        ));
    }

    #[test]
    fn collision_45() {
        let table: Rect = Rect::new(coord! {x:-90.,y:-45.}, coord! {x:90.,y:45.});

        let config = Config {
            ball: point!(x:0.,y:0.),
            angle: 45.,
        };

        let mut collision = tapbiljart::get_collision(&config, &table).unwrap();
        collision.line1.end.y = collision.line1.end_point().y().round();
        assert!(is_colinear(
            &collision.line1,
            &Line::new(config.ball, point!(x:180.,y:180.)),
        ));
        assert!(is_colinear(
            &collision.line2,
            &Line::new(point!(x:-90.,y:45.), point!(x:90.,y:45.))
        ));
    }

    #[test]
    fn bounce_line() {
        let line1 = Line::new(coord! {x:0.,y:0.}, coord! {x:10.,y:10.});
        let line2 = Line::new(coord! {x:10.,y:0.}, coord! {x:10.,y:20.});

        let bounce_line = tapbiljart::get_bounce_line(&Collision { line1, line2 });

        assert!(is_colinear(
            &bounce_line,
            &Line::new(coord! {x:10.,y:10.}, coord! {x:-80.,y:100.})
        ))
    }

    #[test]
    fn bounce_line_vertical() {
        let line1 = Line::new(coord! {x:0.,y:0.}, coord! {x:0.,y:10.});
        let line2 = Line::new(coord! {x:-10.,y:10.}, coord! {x:10.,y:10.});

        let bounce_line = tapbiljart::get_bounce_line(&Collision { line1, line2 });

        assert!(is_colinear(
            &bounce_line,
            &Line::new(coord! {x:0.,y:10.}, coord! {x:0.,y:0.})
        ))
    }

    #[test]
    fn bounce_line_horizontal() {
        let line1 = Line::new(coord! {x:0.,y:10.}, coord! {x:10.,y:10.});
        let line2 = Line::new(coord! {x:10.,y:0.}, coord! {x:10.,y:20.});

        let bounce_line = tapbiljart::get_bounce_line(&Collision { line1, line2 });

        assert!(is_colinear(
            &bounce_line,
            &Line::new(coord! {x:10.,y:10.}, coord! {x:-80.,y:10.})
        ))
    }

    fn is_colinear(line1: &Line, line2: &Line) -> bool {
        let intersection = match line_intersection(*line1, *line2) {
            Some(intersection) => intersection,
            None => return false,
        };

        match intersection {
            geo::LineIntersection::SinglePoint { .. } => {
                dbg!(line1);
                return false;
            }
            geo::LineIntersection::Collinear { .. } => return true,
        }
    }
}
