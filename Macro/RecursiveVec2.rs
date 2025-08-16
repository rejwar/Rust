macro_rules! RecursoveVec {
    ($x:expr) => {
        {
        let mut v = Vec::new();
        v.push($x);
        v
    }
};

($x:expr , $($rest:expr),+)  =>{
    {
        let mut v = RecursoveVec!($($rest), +);
        v.insert(0,$x);
        v
    }
  };
}

fn main() {
    let numbers = RecursoveVec!(1,2,3,4,5,5,6,7);
    println!("Numbers Vec = {:?}", numbers);

    let words = RecursoveVec!("Rust" , "Macro" , "Recursive");
    println!("Words Vec = {:?}", words);

    let tests = RecursoveVec!(String::from("Hello"), String::from("Word"));
    println!("Text vec = {:?}", tests);

    let single = RecursoveVec!(42);
    println!("Single vec = {:?}", single);

    let chars = RecursoveVec!('a' , 'b' , 'c' , 'd');
    println!("chars vec = {:?}", chars);
}