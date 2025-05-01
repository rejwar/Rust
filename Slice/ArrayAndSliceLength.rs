fn main()  {
        let values:[i32; 6] = [4,8,15,16,23,42];

        let RegularReference:&[i32; 6] = &values;
        PrintLength(RegularReference);

        let SliceOfThree: &[i32] = &values [..3];
        PrintLength(SliceOfThree);

}

fn PrintLength(referecne:&[i32]) {
    println!("{}", referecne.len());
}
