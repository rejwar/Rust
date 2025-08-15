macro_rules! my_vec {
    ( $( $x:expr ),* ) => {{
        let mut v = Vec::new();
        $(
            v.push($x);
        )*
        v
    }};
}

fn main() {
    let a = my_vec![10, 20, 30]; // => vec![10,20,30]
}
