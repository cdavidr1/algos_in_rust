fn main() {
    let concat = concat_str("abc", "def");
    let mut p = Person::new();
    p.update_age(1);
    Person::goodbye(p);
}

fn concat_str(a: &str, b: &str) -> String {
    a.to_string() + b
}

struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new() -> Self {
        Self {
            name: String::from("Default"),
            age: 0,
        }
    }

    fn update_age(&mut self, new_age: u8) {
        self.age = new_age;
    }

    fn goodbye(person: Person) {
        println!("Goodbye!");
    }
}
