macro_rules! RepeatRules {
    ($s($msg:expr),*) => {
        $(
            println!("{}", $msg);
        )*
    };
}

fn main() {
    RepeatRules!("One " , "Two" ,"Three");
}
