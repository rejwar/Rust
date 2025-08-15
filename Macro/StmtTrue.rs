macro_rules! CheckAndLogs {
    ($s:stmt) => {
        println!("Checking....!");
        $s
    };
}

fn main() {
    CheckAndLogs!(if true {println!("True");});
;}