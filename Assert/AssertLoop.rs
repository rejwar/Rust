fn main() {


    let mut sum:i32 = 0;

    for i in -3..2 {
        sum += i;
    }

    assert!( sum == -5, "The sum should be -3, but got {}", sum);
    
    for c in 'a'..='z' {
        println!("Character: {}", c);
    }
}
