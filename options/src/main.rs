#![allow(unused_variables, dead_code)]

use std::io;

fn message(addr: String, maybe_payload: Option<Vec<u8>>) -> Result<u64, io::Error> {
    // ...
    if let Some(payload) = maybe_payload {
        Ok(2)
    } else {
        Ok(2)
    }
    // ...
}
fn value_arg<Thing>(thing: Thing) -> Thing {
    thing
}

fn optional_arg<Thing>(thing: Option<Thing>) -> Thing
where
    Thing: std::default::Default,
{
    thing.unwrap_or(Thing::default())
}
fn run() -> Result<u64, io::Error> {
    let a = message("127.0.0.1".to_string(), Some(vec![1, 2, 3]))?;
    Ok(a)
    // message("127.0.0.1".to_string(), None)?;
}
#[derive(Default)]
struct S {
    a: String,
}

fn handles_a_pair_of_stringish<Stringish>(first: Stringish, second: Stringish)
where
    Stringish: AsRef<str>,
{
    // ...
}

fn handles_a_pair_of_varying_stringish(first: impl AsRef<str>, second: impl AsRef<str>) {
    // ...
}

#[test]
fn send_stringishes_of_difference_type() {
    // handles_a_pair_of_stringish("foo".to_string(), "foo"); // This won't compile!
    handles_a_pair_of_varying_stringish("foo".to_string(), "foo");
}

// Separate //
#[derive(Default)]
struct User {
    email: String,
    age: u8,
}

// obviously
impl AsRef<User> for User {
    fn as_ref(&self) -> &User {
        self
    }
}

enum Privilege {
    // imagine different moderator privileges here
}

#[derive(Default)]
struct Moderator {
    user: User,
    privileges: Vec<Privilege>,
}

// since moderators are just regular users
impl AsRef<User> for Moderator {
    fn as_ref(&self) -> &User {
        &self.user
    }
}

fn takes_user<U: AsRef<User>>(user: U) {}

fn main() {
    let ab: S = S { a: "wa".to_owned() };
    let x: Result<u64, io::Error> = run();
    optional_arg(Some(ab));
    println!("Hello, world!");
    let user = User::default();
    let moderator = Moderator::default();

    takes_user(&user);
    takes_user(&moderator); // yay
}
