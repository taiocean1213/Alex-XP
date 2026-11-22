//! `interface!` → `trait`, `class!` → `struct` + `impl`.
//!
//! If a child interface lists a parent (`interface! { Student, Person { ... } }`),
//! rustc requires that parent when you `class!` a type as the child. There is no
//! hardcoded Person/Student special case.

#[macro_export]
macro_rules! interface {
    ($identifier_name:ident) => {
        trait $identifier_name {}
    };
    ($identifier_name:ident, $parent:ident) => {
        trait $identifier_name: $parent {}
    };
    ($identifier_name:ident, $parent:ident { $($body:tt)* }) => {
        trait $identifier_name: $parent {
            $($body)*
        }
    };
    ($identifier_name:ident { $($body:tt)* }) => {
        trait $identifier_name {
            $($body)*
        }
    };
    ($($other:tt)*) => {
        compile_error!(
            "Invalid syntax for interface!. Expected: Name, Name { methods }, Name, Parent, or Name, Parent { methods }"
        );
    };
}

#[macro_export]
macro_rules! class {
    (
        $class_name:ident {
            $($field:ident : $field_type:ty),* $(,)?
        } {
            $($body:tt)*
        }
    ) => {
        struct $class_name {
            $($field: $field_type),*
        }

        impl $class_name {
            $($body)*
        }
    };
    (
        $class_name:ident : [ $($trait_name:ident { $($trait_body:tt)* }),* $(,)? ] {
            $($field:ident : $field_type:ty),* $(,)?
        } {
            $($inherent_body:tt)*
        }
    ) => {
        struct $class_name {
            $($field: $field_type),*
        }

        $(
            impl $trait_name for $class_name {
                $($trait_body)*
            }
        )*

        impl $class_name {
            $($inherent_body)*
        }
    };
    ($($other:tt)*) => {
        compile_error!(
            "Invalid syntax for class!. Expected: Name { fields } { methods } or Name : [Trait { methods }, ...] { fields } { methods }"
        );
    };
}
