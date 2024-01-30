#![allow(dead_code, non_snake_case, unused_variables)]

use std::ffi::c_void;
#[derive(Debug, Default)]
// struct Bar<T> {
// struct Bar<J> {
//     num: i32,
//     random: J,
// }
struct Bar<J> {
    num: i32,
    random: J,
}
pub trait Show<T> {
    type New;
    fn throw(V: T);
}
// impl<J> Bar<J> {
//     fn doa() {
//         println!("awd");
//         println!("awd");
//     }
// }
impl<J, T> Show<T> for Bar<J> {
    type New = String;
    fn throw(V: T) {}
}

trait Pet {
    fn name(&self) -> String;
    fn sound(&self) -> String;
}

struct Cat {
    life: u8, //keeping track of the 9 lives
    age: u8,
    my_name: String,
}

impl Cat {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            my_name: name.into(),
            age: 0,
            life: 0,
        }
    }
}
impl Pet for Cat {
    fn name(&self) -> String {
        (&self.my_name).clone()
    }
    fn sound(&self) -> String {
        "Meow!".into()
    }
}

struct Dog {
    age: u8,
    my_name: String,
}

impl Dog {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            my_name: name.into(),
            age: 0,
        }
    }
}

impl Pet for Dog {
    fn name(&self) -> String {
        (&self.my_name).clone()
    }
    fn sound(&self) -> String {
        "Woof!".into()
    }
}

fn greet_pet(pet: Box<dyn Pet>) {
    println!("You: Hello {}", pet.name());
    println!("{}: {}", pet.name(), pet.sound());
  }

struct Custom<T: ?Sized> {
    num: i32,
    last: T,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct PetVtable {
    drop : fn(*mut c_void),
    size : usize,
    align : usize,
    sound : fn(*const c_void) -> String,
    name : fn(*const c_void) -> String,
}
const POINTER_SIZE : usize = std::mem::size_of::<usize>();

fn main() {
    unsafe {
        // (1)
        let mut kitty : Box<dyn Pet> = Box::new(Cat::new("Kitty"));
        // (2)
        let addr_of_data_ptr = &mut kitty as *mut _ as *mut c_void as usize;
        // (3)
        let addr_of_pointer_to_vtable = addr_of_data_ptr + POINTER_SIZE;
        // (4)
        let ptr_to_ptr_to_vtable = addr_of_pointer_to_vtable as *mut *const PetVtable;
        // (5)
        let mut new_vtable = **ptr_to_ptr_to_vtable; 
        // (6)
        new_vtable.sound = bark;
        // (7)
        *ptr_to_ptr_to_vtable = &new_vtable;

        greet_pet(kitty);
    }

    // let x = "wafa";

    // let mut tom: Box<dyn Pet> = Box::new(Cat::new("Tom"));
    // let mut spike: Box<dyn Pet> = Box::new(Dog::new("Spike"));
    // greet_pet(tom);
    // greet_pet(spike);
    // let a = Bar {
    //     num: 3,
    //     random: 0x22,
    // };

    // println!("Hello, {:?}!", a);
}

fn bark(_this : *const c_void) -> String {
    "Woof!".to_string()
}
