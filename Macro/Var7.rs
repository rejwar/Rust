macro_rules! ShowNamesAndValues {
   ( $( $Name:ident), *) => {
        $(
            println!(" {} = {:?}" , stringify!($Name) , $Name);
        )*
    };
}

fn main() {
    let Var1 =25;
    let Var2 = 88;
    

    ShowNamesAndValues!( Var1, Var2);
}
