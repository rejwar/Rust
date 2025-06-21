macro_rules!  CreateFunction {
    ($name: ident) => {
        fn $name() {
            println!("YOu called {} ()", stringify!($name));
        }
    };
}

CreateFunction!(TestFunction);
CreateFunction!(AnotherFunction);

fn main() {
    TestFunction();
    AnotherFunction();
}
