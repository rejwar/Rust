macro_rules!  PrintSum {
    ($a:expr , $b:expr) => {
        println!("The sum is {}", $a + $b);
    };
}

fn main() {
    PrintSum!(5,6);
    PrintSum!(2+3 , 4*5);
}
