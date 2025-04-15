use std::result;

fn Swap <T, U> (a: T , b: U) ->(U,T){
    (b,a)
}

fn main() {
    let result =Swap(10, "Apple");
    println!("Swapped : {:?}", result);
}
