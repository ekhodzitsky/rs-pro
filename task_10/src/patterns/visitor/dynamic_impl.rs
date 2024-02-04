use std::f64::consts::PI;

pub trait Shape {
    fn accept(&self, visitor: &dyn Visitor) -> f64;
}

pub trait Visitor {
    fn visit_circle(&self, radius: f64) -> f64;
    fn visit_square(&self, side: f64) -> f64;
}

pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Shape for Circle {
    fn accept(&self, visitor: &dyn Visitor) -> f64 {
        visitor.visit_circle(self.radius)
    }
}

pub struct Square {
    side: f64,
}

impl Square {
    pub fn new(side: f64) -> Self {
        Self { side }
    }
}

impl Shape for Square {
    fn accept(&self, visitor: &dyn Visitor) -> f64 {
        visitor.visit_square(self.side)
    }
}

pub struct AreaCalculator;

impl Visitor for AreaCalculator {
    fn visit_circle(&self, radius: f64) -> f64 {
        PI * radius * radius
    }

    fn visit_square(&self, side: f64) -> f64 {
        side * side
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_area_is_correct() {
        let circle = Circle::new(5.0);
        let calculator = AreaCalculator;
        let area = circle.accept(&calculator);
        assert_eq!(area, PI * 5.0 * 5.0);
    }

    #[test]
    fn square_area_is_correct() {
        let square = Square::new(4.0);
        let calculator = AreaCalculator;
        let area = square.accept(&calculator);
        assert_eq!(area, 4.0 * 4.0);
    }
}
