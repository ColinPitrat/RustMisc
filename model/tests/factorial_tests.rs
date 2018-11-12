extern crate model;

#[test]
fn factorial_of_factorial_3() {
    assert_eq!(720, model::factorial(model::factorial(3)));
}
