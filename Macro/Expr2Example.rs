// This macro takes an expression and prints both the expression and its evaluated result
macro_rules! print_expression {
    ($Expression:expr) => {
        println!("Expression: {} = {}", stringify!($Expression), $Expression);
    };
}

fn PrintExpressionResult() {
    print_expression!(5 + 3);           // Expression: 5 + 3 = 8
    print_expression!(10 / 2);          // Expression: 10 / 2 = 5
    print_expression!(2 * (3 + 4));     // Expression: 2 * (3 + 4) = 14
    print_expression!("Hello".len());   // Expression: "Hello".len() = 5
}

fn main() {
    PrintExpressionResult();
}
