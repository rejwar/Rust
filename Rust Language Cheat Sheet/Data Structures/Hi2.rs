#[derive(Debug)]

enum  E {
    A,
    B(),
    C{},
}

fn main() {
    let v1 = E::A;
    let v2 = E::B();
    let v3= E::C {  };

    println!("{:?} {:?} {:?}",v1,v2,v3);
}
