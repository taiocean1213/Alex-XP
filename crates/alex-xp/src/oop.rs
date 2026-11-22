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

/// One class, written once. Fields are `name: Type,`. Methods follow.
///
/// ```ignore
/// obj!(Greeter {
///     who: String,
///     fn new(who: String) -> Self { Self { who } }
///     fn hello(&self) -> String { self.who.clone() }
/// })
/// ```
#[macro_export(local_inner_macros)]
macro_rules! obj {
    ($name:ident { $($body:tt)* }) => {
        obj_items!($name { } { } $($body)*);
    };
    ($($other:tt)*) => {
        ::core::compile_error!("obj! Name { field: Type, fn method(...) { ... } }");
    };
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! obj_items {
    ($struct:ident { $($fields:tt)* } { $($methods:tt)* } fn $mname:ident ($($args:tt)*) -> $ret:ty $body:block $($rest:tt)*) => {
        obj_items!(
            $struct { $($fields)* } { $($methods)* pub fn $mname($($args)*) -> $ret $body }
            $($rest)*
        );
    };
    ($struct:ident { $($fields:tt)* } { $($methods:tt)* } fn $mname:ident ($($args:tt)*) $body:block $($rest:tt)*) => {
        obj_items!(
            $struct { $($fields)* } { $($methods)* pub fn $mname($($args)*) $body }
            $($rest)*
        );
    };
    ($struct:ident { $($fields:tt)* } { $($methods:tt)* } $fname:ident : $fty:ty, $($rest:tt)*) => {
        obj_items!(
            $struct { $($fields)* pub $fname: $fty, } { $($methods)* }
            $($rest)*
        );
    };
    ($struct:ident { $($fields:tt)* } { $($methods:tt)* }) => {
        #[derive(Clone)]
        pub struct $struct { $($fields)* }
        impl $struct { $($methods)* }
    };
    ($($other:tt)*) => {
        ::core::compile_error!(
            "obj! fields are `name: Type,` (comma required), then `fn name(...) { ... }`"
        );
    };
}

/// `iface!(Named { fn name(&self) -> String; })`
/// `iface!(Student: Person { fn university(&self) -> String; })`
#[macro_export(local_inner_macros)]
macro_rules! iface {
    ($name:ident : $parent:ident { $($body:tt)* }) => {
        interface!($name, $parent { $($body)* });
    };
    ($name:ident { $($body:tt)* }) => {
        interface!($name { $($body)* });
    };
    ($($other:tt)*) => {
        ::core::compile_error!("iface!(Name { ... }) or iface!(Child: Parent { ... })");
    };
}
