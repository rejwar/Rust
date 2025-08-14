macro_rules! PrintVarName {
    ($VarName:ident) => {
        println!("Variable name is: {}", stringify!($VarName));
    };
}

fn main() {
    let my_var = 10;
    PrintVarName!(my_var);
}
