struct Commands(Vec<Box<dyn Fn()>>);

impl Commands {
    fn new() -> Commands {
        Commands(Vec::new())
    }

    fn add_command<F>(&mut self, command: F)
    where
        F: Fn() + 'static,
    {
        self.0.push(Box::new(command));
    }

    fn execute_commands(&self) {
        for command in &self.0 {
            command();
        }
    }
}

pub fn demo() {
    println!("Command");

    let mut commands = Commands::new();

    commands.add_command(|| {
        println!("Execute command 1");
    });

    commands.add_command(|| {
        println!("Execute command 2");
    });

    commands.execute_commands();

    println!();
}
