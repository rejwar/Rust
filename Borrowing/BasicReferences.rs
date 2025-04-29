fn main(){
    let x = 10;
    let referenceX = &x;

    println!("X value : {}", x);
    println!("ReferenceX value reference {}", referenceX);
    println!("*referenceX values (dereference) {}",*referenceX );

    let mut y =20;
    let mutable_reference = &mut y;

    println!("Y new value {}", y);
}
