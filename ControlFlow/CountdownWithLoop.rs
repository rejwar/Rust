fn main() {
    let mut seconds: i32 = 10;

    loop {
        if seconds == 0 {
            println!("Blastoff");
            break;
        }

        println!("{seconds } seconds to Blastoff");

        seconds -=1;
    }
}
