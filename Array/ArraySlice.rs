use core::num;

fn ArraySliceExample() {
    let nums = [10,20,30,40];

    let slice = &nums[1..4];

    println!("Slice: {:?}" , slice );
    println!("Length: {}" ,nums.len());
}

fn main () {
    ArraySliceExample();
}
