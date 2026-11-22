//! OOP spectacle: interfaces (traits) and classes (structs) with parent bounds.
//!
//! If you implement only `Student` on a class, rustc errors because
//! `interface! { Student, Person { ... } }` expands to `trait Student: Person`.

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
    ConcretePerson : [Person {
        fn name(&self) -> String {
            self.name.clone()
        }
    }] {
        name: String,
    } {
        fn new(name: String) -> Self {
            Self { name }
        }
    }
}

class! {
    ConcreteStudent : [
        Person {
            fn name(&self) -> String {
                self.name.clone()
            }
        },
        Student {
            fn university(&self) -> String {
                self.university.clone()
            }
        }
    ] {
        name: String,
        university: String,
    } {
        fn new(name: String, university: String) -> Self {
            Self { name, university }
        }
    }
}

pub fn run() {
    let person = ConcretePerson::new(String::from("Alice"));
    let student = ConcreteStudent::new(String::from("Bob"), String::from("MIT"));

    assert_eq!(person.name(), "Alice");
    assert_eq!(student.name(), "Bob");
    assert_eq!(student.university(), "MIT");
    println!("Person name: {}", person.name());
    println!("Student name: {}", student.name());
    println!("Student university: {}", student.university());

    let people: Vec<&dyn Person> = vec![&person, &student];
    let names: Vec<String> = people.iter().map(|p| p.name()).collect();
    assert_eq!(names, ["Alice", "Bob"]);
    for p in people {
        println!("Polymorphic name: {}", p.name());
    }
}

#[allow(dead_code)]
fn main() {
    run();
}
