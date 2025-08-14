macro_rules! PrintMultiple {
    ( $( $Name:ident ),* ) => {
        $(
            println!("{} = {:?}", stringify!($Name), $Name);
        )*
    };
}

fn main() {
    let Age = 25;
    let Score = 88;
    let Year = 2025;

    PrintMultiple!(Age, Score, Year);
}
