use core::task;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn main() {
    let mut hackers: BTreeSet<&str> = BTreeSet::new();

    hackers.insert("Alice");
    hackers.insert("Bob");
    hackers.insert("Charlie");
    hackers.insert("Bob");

    println!("Hackers set : {:?}", hackers);

    if hackers.contains("Alice") {
        println!("Alice is the set");
    }

    for h in &hackers {
        println!("Hacker {}", h);
    }



    hackers.remove("Charlie");
    println!("After removal {:?}",hackers)
}