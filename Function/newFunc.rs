fn main() {
    let val1 = 5;
    let val2 = 2;
    let ans = val1 % val2;
    println!("{}", ans);

    let vec = vec![2,3,4,5,6,7,8,9];
    println!("{:?}", vec);
    vec.pop();
    vec.push(12);
    println!("{:?}", vec);
}