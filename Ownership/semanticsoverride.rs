#[derive(Debug)]
struct MoveBox {
    val: i32,
}

#[derive(Debug, Clone, Copy)]

struct CopyBox {
    val: i32,
}

fn main() {
    let a = MoveBox { val: 20 };
    let b = a;

    println!(" B {:?}", b);

    let c = CopyBox { val: 20 };
    let d = c;

    println!(" C (Original is ){:?}", c);
    println!(" d Original {:?}", d);
}
