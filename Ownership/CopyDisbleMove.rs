#[derive(Debug, Copy, Clone)]
struct Pixed {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let p1 = Pixed {
        r: 10,
        g: 20,
        b: 30,
    };

    let p2 = p1;
    println!("p2: {:?}", p2);

    println!(" Pi is still alive {:?}", p1);
}
