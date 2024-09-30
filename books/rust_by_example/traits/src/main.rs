#![allow(dead_code)]

use rand;


fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");
    dolly.talk();
    dolly.shear();
    dolly.talk();
    dolly.shear();

    // Create random animal
    let animal = random_animal();
    animal.talk();
}


// Returns some struct that implements Animal,
// but we don't know which one at compile time.
fn random_animal() -> Box<dyn Animal> {
    let random_number = rand::random::<f64>();
    if random_number < 0.5 {
        Box::new(Sheep::new("Blue!"))
    } else {
        Box::new(Cow::new("Red"))
    }
}


trait Animal {
    // Associated function
    fn new(name: &'static str) -> Self where Self: Sized;

    // Method signatures
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

struct Cow {
    name: &'static str,
}

struct Sheep {
    naked: bool,
    name: &'static str
}


impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}


impl Animal for Cow {
    fn new(name: &'static str) -> Self {
        Self { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}


impl Animal for Sheep {
    fn new(name: &'static str) -> Self {
        Self { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "Baa?"
        } else {
            "Baa!"
        }
    }

    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}
