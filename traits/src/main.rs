pub trait Trait {
    // always returns i32
    fn returns_num() -> i32;

    // returns implementing type
    fn returns_self() -> Self;
}

struct SomeType;
struct OtherType;

pub struct OtherType2 {
    num: i32,
}

impl Trait for SomeType {
    fn returns_num() -> i32 {
        5
    }

    // Self == SomeType
    fn returns_self() -> Self {
        SomeType
    }
}

impl Trait for OtherType {
    fn returns_num() -> i32 {
        6
    }

    // Self == OtherType
    fn returns_self() -> Self {
        OtherType
    }
}

impl Trait for OtherType2 {
    fn returns_num() -> i32 {
        6
    }

    // Self == OtherType
    fn returns_self() -> Self {
        OtherType2 { num: 2 }
    }
}

trait Trait2 {
    // methods
    fn takes_self(self);
    fn takes_immut_self(&self);
    fn takes_mut_self(&mut self);

    // above methods desugared
    fn takes_self_2(self: Self);
    fn takes_immut_self_2(self: &Self);
    fn takes_mut_self_2(self: &mut Self);
}

// example from standard library
trait ToString {
    fn to_string(&self) -> String;
}
trait Trait3 {
    type AssociatedType;
    fn func(arg: Self::AssociatedType);
}

struct SomeType3;
struct OtherType3;

// any type implementing Trait can
// choose the type of AssociatedType

impl Trait3 for SomeType3 {
    type AssociatedType = i8; // chooses i8
    fn func(arg: Self::AssociatedType) {}
}

impl Trait3 for OtherType3 {
    type AssociatedType = u8; // chooses u8
    fn func(arg: Self::AssociatedType) {}
}

trait Add {
    type Rhs;
    type Output;
    fn add(self, rhs: Self::Rhs) -> Self::Output;
}

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Rhs = Point;
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Separate //
// Separate //
// Separate //
// Separate //
// Separate //

trait Add2<Rhs> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

struct Point2 {
    x: i32,
    y: i32,
}

impl Add2<Point2> for Point2 {
    type Output = Self;
    fn add(self, rhs: Point2) -> Self::Output {
        Point2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add2<i32> for Point2 {
    // ✅
    type Output = String;
    fn add(self, rhs: i32) -> Self::Output {
        "".to_owned()
    }
}

impl Add2<u32> for Point2 {
    // ✅
    type Output = Self;
    fn add(self, rhs: u32) -> Self::Output {
        Point2 {
            x: self.x + rhs as i32,
            y: self.y + rhs as i32,
        }
    }
}

// Separate //
// Separate //
// Separate //
// Separate //
// Separate //

trait Add3<Rhs, Output> {
    fn add(self, rhs: Rhs) -> Output;
}

struct Point3 {
    x: i32,
    y: i32,
}

impl Add3<Point3, Point3> for Point3 {
    fn add(self, rhs: Point3) -> Point3 {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add3<i32, Point3> for Point3 {
    fn add(self, rhs: i32) -> Point3 {
        Point3 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Add3<i32, i32> for Point3 {
    fn add(self, rhs: i32) -> i32 {
        3
    }
}

struct Line {
    start: Point3,
    end: Point3,
}

impl Add3<Point3, Line> for Point3 {
    // ✅
    fn add(self, rhs: Point3) -> Line {
        Line {
            start: self,
            end: rhs,
        }
    }
}
#[derive(Debug)]
struct Xer {
    num: i32,
}
impl Xer {
    fn associated_function() -> Self {
        Xer { num: 2 }
    }

    // fn associated_function() -> Self {
    //     Self { num: 2 }
    // }
    fn trait_method(&self) -> Self {
        Xer { num: 2 }
    }
}
// trait declaration generalized with lifetime & type parameters
trait Trait4<'a, T> {
    // signature uses generic type
    fn func1(arg: T);

    // signature uses lifetime
    fn func2(arg: &'a i32);

    // signature uses generic type & lifetime
    fn func3(arg: &'a T);
}

struct SomeType4;

impl<'a> Trait4<'a, i8> for SomeType4 {
    fn func1(arg: i8) {}
    fn func2(arg: &'a i32) {}
    fn func3(arg: &'a i8) {}
}

impl<'b> Trait4<'b, u8> for SomeType4 {
    fn func1(arg: u8) {}
    fn func2(arg: &'b i32) {}
    fn func3(arg: &'b u8) {}
}

struct Pair<T> {
    x: T,
    y: T,
}
use std::{
    fmt::Display,
    io::{self, Bytes},
};
// impl <T:Display+PartialOrd> for Pair<i32> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

//         Ok(())
//     }
// }

fn test(v: &mut Vec<String>, x: &mut String) {
    // Can I assume here that x is not pointing to v?
}

struct Node<T> {
    elem: T,
}
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
// We *do* have a lifetime here, because Iter has one that we need to define
// impl<'a, T> Iterator for Iter<'a, T> {
//     // Need it here too, this is a type declaration
//     type Item = &'a T;

//     // None of this needs to change, handled by the above.
//     // Self continues to be incredibly hype and amazing
//     fn next(&mut self) -> Option<Self::Item> {
//     //     self.next.map(|node| {
//     //         self.next = node.next.map(|node| &node);
//     //         &node.elem
//     //     })
//     }
// }

fn main() {
    let mut f = "2".to_owned();
    let mut g = "2".to_owned();
    let mut h = vec![f, g];
    // test(&mut h, &mut f);
    // let bytes = Bytes::new();
    // let z = None::<!>?;

    let y = Xer { num: 3 };
    println!("{:?}", y);

    // you can use :: for both methods and associated functions
    println!("{:?}", Xer::associated_function());
    println!("{:?}", Xer::trait_method(&y));

    // you can use . only for methods
    println!("{:?}", y.trait_method());

    // this doesnt work
    // println!("{:?}", y.associated_function());

    let a: OtherType2 = OtherType2 { num: 2 };
    OtherType2::returns_num();

    SomeType3::func(-1_i8); // can only call func with i8 on SomeType
    OtherType3::func(1_u8); // can only call func with u8 on OtherType

    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    let p3 = p1.add(p2);
    // similar
    let p4 = Point { x: 1, y: 1 };
    let p5 = Point { x: 2, y: 2 };
    let p6 = Point::add(p4, p5);

    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);
    println!("Hello, world!");
}
