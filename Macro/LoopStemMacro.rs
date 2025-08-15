macro_rules! repeat_twice {
    ($s:stmt) => {
        $s
        $s
    };
}

fn main() {
    repeat_twice!(for i in 0..1 { println!("Hello"); });
}
