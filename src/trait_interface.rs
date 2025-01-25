trait Greet{
    fn say_hello(&self);
}

struct Person {
    name: String
}

impl Greet for Person {
    fn say_hello(&self) {
        println!("Hello, my name is {}", self.name)
    }
}

pub fn trait_interface_fn() {
    println!("----------------------------------> Trait Interface");
    let person = Person {
        name: String::from("Trait Interface")
    };

    person.say_hello();
}