let x = Some(5);
println!("{}", x.unwrap_or_else(|| 10)); // ✅ 5

let y: Option<i32> = None;
println!("{}", y.unwrap_or_else(|| {
    println!("Calculating fallback...");
    10
})); // 
