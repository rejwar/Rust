fn main() {
    let mut count = 0;

    for num in 0..21 {
        if num % 2 ==0 {
            continue;
        }
        count+=1;
    }
    println!(" The count of odd values between 0 and 20 is : {}", count);
}
