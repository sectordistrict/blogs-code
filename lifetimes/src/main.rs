use std::intrinsics::mir::Variant;

fn function1<'a>() -> &'a String {
    &("wfawf".to_owned())
}

fn function2<'a>(x: &'a str) {}

// Single element
// Data of x should live at least as long as Struct exists
struct Struct1<'a> {
    x: &'a str,
}

// Multiple elements
// Data of x and y should live at least as long as Struct exists
struct Struct2<'a> {
    x: &'a str,
    y: &'a str,
}

// Variant with a single element
// Data of the variant should live at least as long as Enum exists

fn main() {
    println!("{}", function1());
}
