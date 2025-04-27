use std::result;

fn main() {
    let mut result = String::new();

    for i in 'a' ..='t'{
        result.push(i);
}
println!("Front :{}" , result);
}
