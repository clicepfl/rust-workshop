#![allow(dead_code)]

use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
struct Person { name: String }

fn greet(person: &Person) {
    // Deref coercion: those are equivalent
    // The compiler automatically dereferences things when needed
    println!("hello {}", person.name);
    println!("hello again {}", (*person).name);
    //println!("goodbye {}", person.deref().name); // Also possible but rarely useful
}

// Smart pointers: safe way to access data on the heap
fn box_intro() {
    let person: Person = Person { name: String::from("John Doe") };
    greet(&person);

    let boxed_person: Box<_> = Box::new( Person { name: String::from("Jane Smith") } );
    greet(&boxed_person);   // What?
}



enum LinkedListNode {
    None,
    // This causes an error, because the memory footprint of LinkedListNode is unknowable
    //Next(i32, LinkedListNode)
    // Instead, we can do this
    Next(i32, Box<LinkedListNode>)   // It works, because Box is just a pointer, it has a fixed size
}

fn linked_list() {
    let my_list = LinkedListNode::Next(3, Box::new(LinkedListNode::Next(5, Box::new(LinkedListNode::Next(7, Box::new(LinkedListNode::None))))));
    if let LinkedListNode::Next(value, _) = my_list {
        println!("{value}");
    }
}



fn box_dyn() {
    let person: Person = Person { name: String::from("Alex Taylor") };

    // Boxes allow for run-time polymorphism (as opposed to compile-time with generics)
    // Don't do this unless you have a good reason to
    let displayables: Vec<Box<dyn Debug>> = vec![
        Box::new(1),
        Box::new(2.0),
        Box::new("hello"),
        Box::new([1, 2, 3]),
        Box::new(person),
    ];

    for i in displayables { println!("{:?}", i); }

    // Also, nothing is stopping us from
    //let oh_no: Box<dyn Any> = ...;
    // But you should probably _never_ do this unless you have a very good reason to
}



fn main() {
    box_intro();
    linked_list();
    box_dyn();
}

