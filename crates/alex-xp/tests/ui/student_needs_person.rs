use alex_xp::*;

interface! {
    Person {
        fn name(&self) -> String;
    }
}

interface! {
    Student, Person {
        fn university(&self) -> String;
    }
}

class! {
    BadStudent : [Student {
        fn university(&self) -> String {
            self.university.clone()
        }
    }] {
        name: String,
        university: String,
    } {
        fn new(name: String, university: String) -> Self {
            Self { name, university }
        }
    }
}

fn main() {}
