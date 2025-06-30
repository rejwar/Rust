#[derive(Debug)]
enum E {
    A,
    B(i32, i32),
    C{x:i32, y:i32},
}

fn MatchEnumVarients(e : E){
    match e {
        E::A => println!("Matched unit Varient"),
        E::B(a, b) => println!("Matched Tuple varient {} {}",a,b),
        E::C { x,y } => println!("Matched Struct vaient {} {}",x,y),
    }
}

fn main() {
    let v1 = E::A;
    let v2 = E::B(4, 6);
    let v3 = E::C { x: 4, y: 10 };

    MatchEnumVarients(v1);
    MatchEnumVarients(v2);
    MatchEnumVarients(v3);
}
