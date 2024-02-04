mod patterns;

fn main() {
    println!("Demonstrating Decorator Pattern:");
    demonstrate_decorator();

    println!("\nDemonstrating Composite Pattern:");
    demonstrate_composite();

    println!("\nDemonstrating Visitor Pattern:");
    demonstrate_visitor();
}

fn demonstrate_decorator() {
    use patterns::decorator::{dynamic_impl, static_impl, Coffee};

    println!("Static Typing:");
    {
        let coffee = static_impl::SimpleCoffee {};
        println!("Simple coffee cost: {}", coffee.cost());

        let milk_coffee = static_impl::MilkDecorator::new(coffee);
        println!("Coffee with milk cost: {}", milk_coffee.cost());

        let sweet_milk_coffee = static_impl::SugarDecorator::new(milk_coffee);
        println!(
            "Coffee with milk and sugar cost: {}",
            sweet_milk_coffee.cost()
        );
    }

    println!("\nDynamic Typing:");
    {
        let coffee: Box<dyn Coffee> = Box::new(dynamic_impl::SimpleCoffee {});
        println!("Simple coffee cost: {}", coffee.cost());

        let milk_coffee = dynamic_impl::MilkDecorator::new(coffee);
        println!("Coffee with milk cost: {}", milk_coffee.cost());

        let sweet_milk_coffee = dynamic_impl::SugarDecorator::new(milk_coffee);
        println!(
            "Coffee with milk and sugar cost: {}",
            sweet_milk_coffee.cost()
        );
    }
}

fn demonstrate_composite() {
    use patterns::composite::dynamic_impl::{
        Directory as DynDirectory, File as DynFile, FileSystemComponent as DynFileSystemComponent,
    };
    use patterns::composite::static_impl::FileSystemComponent::{Directory, File};

    println!("\nDemonstrating Composite Pattern with Static Typing:");
    let static_file_system = Directory {
        name: "root".to_string(),
        components: vec![
            File {
                name: "static_file1.txt".to_string(),
                size: 100,
            },
            Directory {
                name: "static_subdir".to_string(),
                components: vec![File {
                    name: "static_subfile1.txt".to_string(),
                    size: 150,
                }],
            },
        ],
    };
    println!(
        "Total size of static file system: {}",
        static_file_system.get_size()
    );

    println!("\nDemonstrating Composite Pattern with Dynamic Typing:");
    let mut dynamic_root = DynDirectory::new("root");
    dynamic_root.add_component(Box::new(DynFile {
        name: "dynamic_file1.txt".to_string(),
        size: 100,
    }));
    let mut dynamic_subdir = DynDirectory::new("dynamic_subdir");
    dynamic_subdir.add_component(Box::new(DynFile {
        name: "dynamic_subfile1.txt".to_string(),
        size: 150,
    }));
    dynamic_root.add_component(Box::new(dynamic_subdir));

    println!(
        "Total size of dynamic file system: {}",
        dynamic_root.get_size()
    );
}

fn demonstrate_visitor_static() {
    use patterns::visitor::static_impl::{AreaCalculator, Shape, Visitor};

    let shapes = vec![Shape::Circle(5.0), Shape::Square(10.0)];

    let calculator = AreaCalculator;

    for shape in shapes.iter() {
        let area = calculator.visit(shape);
        println!("Shape area: {}", area);
    }
}

fn demonstrate_visitor_dynamic() {
    use patterns::visitor::dynamic_impl::{AreaCalculator, Circle, Shape, Square};

    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle::new(5.0)), Box::new(Square::new(10.0))];

    let calculator = AreaCalculator;

    for shape in shapes.iter() {
        let area = shape.accept(&calculator);
        println!("Dynamic Shape area: {}", area);
    }
}

fn demonstrate_visitor() {
    println!("\nDemonstrating Visitor Pattern with Static Typing:");
    demonstrate_visitor_static();

    println!("\nDemonstrating Visitor Pattern with Dynamic Typing:");
    demonstrate_visitor_dynamic();
}
