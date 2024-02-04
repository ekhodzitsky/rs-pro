use super::Coffee;

pub struct SimpleCoffee;

impl Coffee for SimpleCoffee {
    fn cost(&self) -> u32 {
        10
    }
}

pub struct MilkDecorator {
    component: Box<dyn Coffee>,
}

impl MilkDecorator {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(component: Box<dyn Coffee>) -> Box<dyn Coffee> {
        Box::new(MilkDecorator { component })
    }
}

impl Coffee for MilkDecorator {
    fn cost(&self) -> u32 {
        self.component.cost() + 2
    }
}

pub struct SugarDecorator {
    component: Box<dyn Coffee>,
}

impl SugarDecorator {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(component: Box<dyn Coffee>) -> Box<dyn Coffee> {
        Box::new(SugarDecorator { component })
    }
}

impl Coffee for SugarDecorator {
    fn cost(&self) -> u32 {
        self.component.cost() + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_coffee() {
        let coffee: Box<dyn Coffee> = Box::new(SimpleCoffee {});
        assert_eq!(coffee.cost(), 10);
    }

    #[test]
    fn test_milk_decorator() {
        let coffee: Box<dyn Coffee> = Box::new(SimpleCoffee {});
        let milk_coffee = MilkDecorator::new(coffee);
        assert_eq!(milk_coffee.cost(), 12);
    }

    #[test]
    fn test_sugar_decorator() {
        let coffee: Box<dyn Coffee> = Box::new(SimpleCoffee {});
        let sugar_coffee = SugarDecorator::new(coffee);
        assert_eq!(sugar_coffee.cost(), 11);
    }

    #[test]
    fn test_milk_and_sugar_decorator() {
        let coffee: Box<dyn Coffee> = Box::new(SimpleCoffee {});
        let milk_coffee = MilkDecorator::new(coffee);
        let sweet_milk_coffee = SugarDecorator::new(milk_coffee);
        assert_eq!(sweet_milk_coffee.cost(), 13);
    }
}
