macro_rules! define_item {
    ($i:item) => {
        $i
    };
}

define_item! {
    fn hello() {
        println!("Hello from macro!");
    }
}

fn main() {
    hello();
}
