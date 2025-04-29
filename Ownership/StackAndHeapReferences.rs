fn main() {
    let MyStackValue:i32 = 2;
    let MyIntegerReference:&str =&MyStackValue;

    let MyHeapValue:String = String::from("Toyota");
    let MyHeapReference:&String = &MyHeapValue;
}
