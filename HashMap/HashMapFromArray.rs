use std::collections::HashMap;


fn main() {
    let data:=[
     ("Bobby" ,7),
     ("Grant" , 4),
     ("Ben" , 6),

    ];


    let YearsAtCompany = HashMap::from(data);
    println!("{:?}" , YearsAtCompany);
}
