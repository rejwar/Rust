fn main() {
    let mut seconds: i32 = 10;

    loop {
        if seconds == 0 {
            println!("Blastoff");
            break;
        }

        if seconds % 2 == 0 {
            println!("{} seconds (even number ), skipping 3 seconds", seconds );
        }



        println!("{seconds } seconds to Blastoff");
        seconds -=1;
        
    }
}

