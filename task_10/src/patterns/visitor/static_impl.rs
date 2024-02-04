use std::f64::consts::PI;

pub enum Shape {
    Circle(f64),
    Square(f64),
}

pub trait Visitor<T> {
    fn visit(&self, shape: &Shape) -> T;
}

pub struct AreaCalculator;

impl Visitor<f64> for AreaCalculator {
    fn visit(&self, shape: &Shape) -> f64 {
        match shape {
            Shape::Circle(radius) => PI * radius * radius,
            Shape::Square(side) => side * side,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_of_circle() {
        let circle = Shape::Circle(10.0);
        let calculator = AreaCalculator;
        let area = calculator.visit(&circle);
        assert_eq!(area, PI * 10.0 * 10.0);
    }

    #[test]
    fn area_of_square() {
        let square = Shape::Square(10.0);
        let calculator = AreaCalculator;
        let area = calculator.visit(&square);
        assert_eq!(area, 10.0 * 10.0);
    }
}
