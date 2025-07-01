struct Pair (i32, i32);

fn main() {
    let a = 10;
    let b = 20;


    let p = Pair { 0: a , 1:b};

    println!(" pair {} {}" , p.0 , p.1);
}
