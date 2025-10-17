// Defines a macro to create traits with optional inheritance and method signatures.
macro_rules! interface {
	// Empty trait with no parent or methods.
	($identifier_name:ident) => {
		trait $identifier_name {}
	};
	// Empty trait with a parent trait, inheriting its methods.
	($identifier_name:ident, $parent:ident) => {
trait $identifier_name:
		$parent {}
	};
	// Trait with method signatures but no parent.
	($identifier_name:ident { $($body:tt)* }) => {
		trait $identifier_name {
			$($body)*
		}
	};
	// Trait with method signatures and a parent trait, inheriting its methods.
	($identifier_name:ident, $parent:ident { $($body:tt)* }) => {
trait $identifier_name:
		$parent {
			$($body)*
		}
	};
}

// Defines a macro to create structs with fields and implement traits or inherent methods.
// Includes compile-time checks to ensure parent traits are implemented when child traits are used.
macro_rules! class {
	// Creates a struct with fields and inherent methods (no traits).
	// Syntax: class!{StructName { field: Type, ... } { fn method(...) { ... } }}
	($class_name:ident { $($field:ident : $field_type:ty),* } { $($body:tt)* }) => {
		// Define the struct with the specified fields.
		struct $class_name {
			$($field: $field_type),*
		}

		// Implement inherent methods for the struct.
		impl $class_name {
			$($body)*
		}
	};
	// ERROR CASE: Implementing Student without Person - triggers compile error.
	// This arm catches attempts to implement only Student and errors at macro expansion time.
	($class_name:ident : [Student { $($trait_body:tt)* }] { $($field:ident : $field_type:ty),* } { $($inherent_body:tt)* }) => {
		compile_error!(
		    "When implementing the child trait 'Student', you must also explicitly implement its parent trait 'Person'. \
             Add 'Person { ... }' to the trait list: [Person { ... }, Student { ... }]"
		);
	};
	// Creates a struct with fields, implements one or more traits, and adds inherent methods.
	// Syntax: class!{StructName : [Trait { fn method(...) { ... } }, ...] { field: Type, ... } { fn inherent(...) { ... } }}
	($class_name:ident : [ $($trait_name:ident { $($trait_body:tt)* }),* ] { $($field:ident : $field_type:ty),* } { $($inherent_body:tt)* }) => {
		// Define the struct with the specified fields.
		struct $class_name {
			$($field: $field_type),*
		}

		// Implement each specified trait with its own method bodies.
		$(
		impl $trait_name for $class_name {
		$($trait_body)*
		}
		)*

		// Implement inherent methods for the struct.
		impl $class_name {
			$($inherent_body)*
		}
	};
}

// Define a Person trait with a name method.
interface! {
	Person {
		fn name(&self) -> String;
	}
}

// Define a Student trait that inherits from Person and adds a university method.
interface! {
	Student, Person {
		fn university(&self) -> String;
	}
}

// Define a ConcretePerson struct implementing the Person trait with a constructor.
class! {
ConcretePerson :
	[Person {
		       // Implement the name method for the Person trait.
		       fn name(&self) -> String {
			       self.name.clone()
		       }
	       }] {
		// Define the struct's field.
name:
		String
	} {
		// Inherent constructor method to create a new ConcretePerson.
		fn new(name: String) -> Self {
			Self { name }
		}
	}
}

// Define a ConcreteStudent struct implementing Person and Student traits with a constructor.
// Note: Both Person and Student must be listed to satisfy inheritance.
class! {
ConcreteStudent :
	[Person {
		       // Implement the name method for the Person trait (inherited via Student).
		       fn name(&self) -> String {
			       self.name.clone()
		       }
	       }, Student {
		       // Implement the university method for the Student trait.
		       fn university(&self) -> String {
			self.university.clone()
		}
	}] {
		// Define the struct's fields.
name:
		String,
university:
		String
	} {
		// Inherent constructor method to create a new ConcreteStudent.
		fn new(name: String, university: String) -> Self {
			Self { name, university }
		}
	}
}

// BAD EXAMPLE (commented out): This will trigger a compile error if uncommented.
// class!{BadStudent : [Student {
//     fn university(&self) -> String {
//         self.university.clone()
//     }
// }] {
//     name: String,
//     university: String
// } {
//     fn new(name: String, university: String) -> Self {
//         Self { name, university }
//     }
// }}

// Example usage demonstrating trait inheritance and inherent methods.
fn main() {
	// Create instances using inherent constructor methods.
	let person = ConcretePerson::new(String::from("Alice"));
	let student = ConcreteStudent::new(String::from("Bob"), String::from("MIT"));

	// Demonstrate Person trait methods.
	println!("Person name: {}", person.name());

	// Demonstrate Student trait methods, including inherited name() from Person.
	println!("Student name: {}", student.name()); // Inherited from Person via Student
	println!("Student university: {}", student.university());

	// Demonstrate polymorphism with trait objects to show inheritance.
let people:
	Vec<&dyn Person> = vec![&person, &student];
	for p in people {
	println!("Polymorphic name: {}", p.name());
	}
}