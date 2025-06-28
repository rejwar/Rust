fn main() {
    let nums = vec![1,2,3,4,5,6,7,8];

    let evens: Vec<i32> = nums
    .into_iter().filter(|x| x % 2 == 0).collect();
    {
            println!( "{:?}", evens);
    } 
}
