use core::num;

fn main() {
    let  mut nums = vec![1,2,3];

    nums.push(4);
    println!("{:?}", nums);

    nums.pop();
    println!("{:?}", nums);

    let mut vec = Vec::new();
    vec.push("Test");
    vec.push("String");
    println!("{:?}", vec);
}


