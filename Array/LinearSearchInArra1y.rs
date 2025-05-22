use std::{collections::btree_set, ops::Index};

fn main() {
    let Numbers: [i32; 7] = [10,20,30,40,50,60,70];
    let TargetElement: i32 = 20;
    let mut FoundIndex: Option<usize> = None;

    for Index in 0..Numbers.len() {
        if Numbers[Index] == TargetElement{
            FoundIndex = Some(Index);
            break;
        }
    }

    match FoundIndex {
        Some(Index) =>println!("Element {} found at index {}", TargetElement , Index),
        None => println!("Element {} not found in the array " , TargetElement),
    }

    let AnotherTarget : i32 = 100;
    FoundIndex = None;
    for Index in 0..Numbers.len() {
        if Numbers[Index] == AnotherTarget {
            FoundIndex = Some(Index);
            break;
        }
    }
    match FoundIndex {
        Some(Index) => println!("Element {} found at index {}", AnotherTarget,Index),
        None => println!("Element {} not found in the array " , AnotherTarget),
    }
}
