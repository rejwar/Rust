fn main() {
    let MyStackValue:i32 = 2;
    let MyIntegerReference:&i32 = &MyStackValue;
    println!("{}", MyIntegerReference);

    let MyHeapValue:String = String::from("Toyota");
    let MyHeapReference:&String = &MyHeapValue;
    println!("{}", MyHeapReference);
}
