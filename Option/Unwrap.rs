let x = Some(5);
println!("{}", x.unwrap()); // ✅ 5 প্রিন্ট করবে

let y: Option<i32> = None;
println!("{}", y.unwrap()); // ❌ panic হবে
