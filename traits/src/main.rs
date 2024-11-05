mod basket;
mod container;
mod stack;

use basket::Basket;
use stack::Stack;

fn main() {
    let b1 = Basket::new(String::from("Hi there!"));
    let b2 = Basket::new(4);
    let b3 = Basket::new(false);

    let s1 = Stack::new(vec![String::from("Hi there!")]);
    let s2 = Stack::new(vec![4]);
    let s3 = Stack::new(vec![false]);
}
