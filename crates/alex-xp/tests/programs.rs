//! The packaged examples are not part of `cargo test` on their own.
//! Compiling them here runs each `main`, including the assertions in that file.

#[path = "../examples/auto_tour.rs"]
mod auto_tour;
#[path = "../examples/drill_chain.rs"]
mod drill_chain;
#[path = "../examples/owned_conversion.rs"]
mod owned_conversion;
#[path = "../examples/ownership_tour.rs"]
mod ownership_tour;
#[path = "../examples/person_student.rs"]
mod person_student;

#[test]
fn auto_tour() {
    auto_tour::run();
}

#[test]
fn drill_chain() {
    drill_chain::run();
}

#[test]
fn owned_conversion() {
    owned_conversion::run();
}

#[test]
fn ownership_tour() {
    ownership_tour::run();
}

#[test]
fn person_student() {
    person_student::run();
}
