// macro_rules! PrintSum {
//     ($a: expr , $b: expr) => {
//         println!("The sum is {}" , $a + $b);
//     };
// }
// fn main() {
//     PrintSum!(  5,10);
//     PrintSum!( 2+3 , 5*6);
// }

macro_rules! PrintSum {
    ($a: expr, $b: expr) => {
        println!("The number is {}" , $a +$b);
    };
}

fn main() {
    PrintSum!(5,10);
    PrintSum!(2+3 , 5*6);
}
