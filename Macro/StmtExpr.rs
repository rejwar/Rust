macro_rules! expr_macro {
    ($e:expr) => {
        println!("Value: {}", $e);
    };
}

macro_rules! stmt_macro {
    ($s:stmt) => {
        $s
    };
}

fn main() {
    expr_macro!(5 + 3);       
    stmt_macro!(let x = 10;); 
}
