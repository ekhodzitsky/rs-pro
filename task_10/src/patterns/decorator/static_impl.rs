use super::Coffee;

pub struct SimpleCoffee;

impl Coffee for SimpleCoffee {
    fn cost(&self) -> u32 {
        10
    }
}

pub struct MilkDecorator<T: Coffee> {
    component: T,
}

impl<T: Coffee> MilkDecorator<T> {
    pub fn new(component: T) -> Self {
        MilkDecorator { component }
    }
}

impl<T: Coffee> Coffee for MilkDecorator<T> {
    fn cost(&self) -> u32 {
        self.component.cost() + 2
    }
}

pub struct SugarDecorator<T: Coffee> {
    component: T,
}

impl<T: Coffee> SugarDecorator<T> {
    pub fn new(component: T) -> Self {
        SugarDecorator { component }
    }
}

impl<T: Coffee> Coffee for SugarDecorator<T> {
    fn cost(&self) -> u32 {
        self.component.cost() + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_coffee() {
        let coffee = SimpleCoffee {};
        assert_eq!(coffee.cost(), 10);
    }

    #[test]
    fn test_milk_decorator() {
        let coffee = SimpleCoffee {};
        let milk_coffee = MilkDecorator::new(coffee);
        assert_eq!(milk_coffee.cost(), 12);
    }

    #[test]
    fn test_sugar_decorator() {
        let coffee = SimpleCoffee {};
        let sugar_coffee = SugarDecorator::new(coffee);
        assert_eq!(sugar_coffee.cost(), 11);
    }

    #[test]
    fn test_milk_and_sugar_decorator() {
        let coffee = SimpleCoffee {};
        let milk_coffee = MilkDecorator::new(coffee);
        let sweet_milk_coffee = SugarDecorator::new(milk_coffee);
        assert_eq!(sweet_milk_coffee.cost(), 13);
    }
}
