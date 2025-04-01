fn main()
{
    let numbers = vec![1,2,3,4,5,6,7,8,9];

    let even_numbers: Vec<i32> = numbers.iter().filter(|&x| x%2 ==0).cloned().collect();
    println!("Even Numbers: {:?}" , even_numbers);

    let squared_numbers: Vec<i32> = numbers.iter().map(|x|x*x).collect();
    println!("Squared Numbers:{:?}", squared_numbers);
}
