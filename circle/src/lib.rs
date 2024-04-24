use std::f64::consts::PI;

#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Circle {
            center: Point { x, y },
            radius: r,
        }
    }

    pub fn diameter(&self) -> u64 {
        (2.0 * self.radius) as u64
    }
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    pub fn intersect(&self, circle: &Circle) -> bool {
        circle.center.distance(&self.center) < self.radius + circle.radius
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(&self, point: &Point) -> f64 {
        let (dx, dy) = (point.x - self.x, point.y - self.y);
        (dx * dx + dy * dy).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_methods() {
        let point_a = Point { x: 1.0, y: 1.0 };
        let point_b = Point { x: 0.0, y: 0.0 };
        let result = point_a.distance(&point_b);
        assert_eq!(result, 1.4142135623730951)
    }

    #[test]
    fn test_circle_methods() {
        let circle = Circle::new(500.0, 500.0, 150.0);
        let circle1 = Circle {
            center: Point { x: 80.0, y: 115.0 },
            radius: 30.0,
        };
        let result = circle.area();
        assert_eq!(result, 70685.83470577035);

        let result = circle.diameter();
        assert_eq!(result, 300);

        let result = circle1.diameter();
        assert_eq!(result, 60);

        let result = circle.intersect(&circle1);
        assert_eq!(result, false);
    }
}
