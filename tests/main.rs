#[cfg(test)]
mod tests {
    use geo::{coord, point, Line, Rect};
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
        assert_eq!(
            collision,
            Collision {
                line1: Line::new(config.ball, point!(x:0.,y:-45.)),
                line2: Line::new(point!(x:90.,y:-45.), point!(x:-90.,y:-45.))
            }
        )
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
        assert_eq!(
            collision,
            Collision {
                line1: Line::new(config.ball, point!(x:180.,y:180.)),
                line2: Line::new(point!(x:-90.,y:45.), point!(x:90.,y:45.))
            }
        )
    }
}
