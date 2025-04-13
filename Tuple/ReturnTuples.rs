fn CalcualteStats(Numbers:&[i32]) -> (i32, i32){
    let sum: i32 = Numbers.iter().sum();
    let count = Numbers.len() as i32 ;
    (sum , count)
}

fn main() 
{
    let (total,length) = CalcualteStats(&[10,20,30]);
    println!("Sum: {} , Count: {}", total,length);
}
