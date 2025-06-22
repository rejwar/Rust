use std::option;

fn main() {
    let mut optional = Some(5);

    while let Some(i) = optional {
        if i > 3 {
            println!("Too big ({}) , stopping " , i);
            optional = None;
        } else {
            println!(" i = {} , continue ",i);
            optional = Some(i + 2 );
        }
    }
}
