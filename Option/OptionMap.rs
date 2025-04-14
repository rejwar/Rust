fn main(){
    let num =Some(5);
    let squared = num.map(|x|x *x);
    println!("Squared: {:?}", squared);

    let none_val: Option<i32>= None;
    let result = none_val.map(|x|x +1);
    println!("Result: {:?}",result);
}
