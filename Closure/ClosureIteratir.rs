fn main() {
    let Numbers = vec![1,2,3,4,5,6];
    let Doubled: Vec<_> = Numbers.iter().map(|x| x *2 ).collect();

    println!("Doubled {:?}", Doubled );
}