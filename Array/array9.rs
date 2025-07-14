fn main() {
    let mut xs: [i32 ; 5] = [1,2,3,4,5];
    println!("Length of xs : {}", xs.len());
    println!("Data siZe of xs is {}",size_of_val(&xs));

    let slice_xs: &[i32] = &xs[1..4];

    println!("Slice of xs : {:?}", slice_xs);
    println!("Slice of xs : {:?}", &xs[1..=4]);
    println!("Slice of xs : {:?}", &xs[1..5]);
}