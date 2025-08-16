macro_rules! RecursiveVec {
    ($x: expr) => {
        
        {
            let mut v = Vec::new();
            v.push($x);
            v
        }
    };

    ($x: expr ,$($rest:expr), +) => {
        {
        let mut v = RecursiveVec! ($($rest), +);
        v.insert(0,$x);
        v
    }
};

}


fn main() {
    let my_vec = RecursiveVec!(10,20,30,40);
    println!("Recursive Vec ! Result = {:?}", my_vec);

    let single_vec = RecursiveVec!(99);
    println!("Recursive single = {:?}", single_vec);
}