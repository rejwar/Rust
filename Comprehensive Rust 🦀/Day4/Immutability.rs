

fn main() {
    let number = vec![1,2,3,4,5,6,7,8];
    let mut doubled = Vec::new();

    for num in number {
        doubled.push(num *2);
    }


    println!("{:?}",doubled);
}
