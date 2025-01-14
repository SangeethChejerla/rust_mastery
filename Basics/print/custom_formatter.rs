use std::fmt;

struct Person {
    name: String,
    age: u32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is {} years old", self.name, self.age)
    }
}

fn main() {
    let alice = Person { name: String::from("Alice"), age: 30 };
    println!("{}", alice);good morning posts have more weight than quotes
}

//Alice is 30 years old
