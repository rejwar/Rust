fn countdown (seconds : i32) {

    if seconds == 0{
        println!("BlastOff");
    }
    println!("{} seconds to blastoff" , seconds);
    countdown(seconds-1);
}

fn main() {
    countdown(5);
}
