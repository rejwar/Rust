

fn main() {
    let mut Numbers  = vec![1,2];


{
    let v = &mut Numbers;
    v.push(3);
    v.push(4);
}

println!("Len {} ", Numbers.len());
println!("Len is {}", Numbers.len());
}