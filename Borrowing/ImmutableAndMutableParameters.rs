fn UpdateValues(ConstValue : i32 , MutValue : &mut i32 ) {
    println!("Immutable Value : {}", ConstValue);
    *MutValue +=10;
}

fn main() {
    let Constant: i32 = 30;
    let mut Variable : i32 = 40;

    UpdateValues(Constant,&mut Variable);
    println!("Updated Mutable Value :{}",Variable);
}
