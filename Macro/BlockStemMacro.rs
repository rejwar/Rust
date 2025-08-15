macro_rules! block_macro {
    ($s:stmt) => {
        println!("Running block:");
        $s
    };
}

fn main() {
    block_macro!({
        println!("Inside block");
    });
}
