macro_rules! RunBlock {
    ($b:block) => {
        println!("Running Block");
        $b
    };
}

fn main() {
    RunBlock!( {
        let x =10;
        println!("x = {}", x);
    });
}