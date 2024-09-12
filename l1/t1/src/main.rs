pub trait Action {
    fn say(&self) -> ();
}

struct Person {
    name: String
}

impl Action for Person {
    fn say(&self) -> () {
        println!("Hello, {}", &self.name)
    }
}

impl Person {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string()
        }
    }
}

fn main() {
    let roman = Person::new("Roman");
    roman.say();
}
