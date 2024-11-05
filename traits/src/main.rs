mod basket;
mod container;
mod stack;

use basket::Basket;
use container::Container;
use stack::Stack;

fn add_string<T: Container<String>>(container: &mut T, value: String) {
    container.put(value);
}

fn main() {
    let mut b1 = Basket::new(String::from("Hi there!"));
    let b2 = Basket::new(4);
    let b3 = Basket::new(false);

    let mut s1 = Stack::new(vec![String::from("Hi there!")]);
    let s2 = Stack::new(vec![4]);
    let s3 = Stack::new(vec![false]);

    add_string(&mut b1, String::from("Some String"));
    add_string(&mut s1, String::from("Some String"));
}
