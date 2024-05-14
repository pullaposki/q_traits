struct Animal {
    name: String,
}

trait Dog {
    fn bark(&self) {
        println!("Woof!");
    }

    fn run(&self) {
        println!("The dog is running");
    }
}

impl Dog for Animal {}

fn main() {
    let rover = Animal {
        name:"Rover".to_string(),
    };

    rover.run();
    rover.bark();
}
