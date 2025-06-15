fn print_slice(slice: &[i32])
{
    for item in slice{
        println!( "{}", item);
    }
}

fn main() {
    let data = [1,2,3,4,5,6];
    print_slice(&data[2..]);
}
