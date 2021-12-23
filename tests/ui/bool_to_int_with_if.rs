#![warn(clippy::bool_to_int_with_if)]
fn foo() {}
fn bar() {}
fn main() {
    let condition = true;
    // catch this
    if condition {
        1
    } else {
        0
    }

    //Don't catch this
    if condition {
        foo()
    } else {
        bar()
    }

    if condition { 1 } else { 2 }
}
