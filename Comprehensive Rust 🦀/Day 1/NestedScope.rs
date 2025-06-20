fn  main() {
    let a = 1;
    {
        let b = 2;
        {
            let c = 3;
            println!("a: {}, b: {}, c: {}", a, b, c);
        }
        // c is not accessible here
        println!("a: {}, b: {}", a, b);
    }
    println!("a: {}", a);
    // b is not accessible here
}
