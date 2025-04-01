fn GetMessage() -> &' static str {
"This is a static string"
}


fn main() {
let msg = GetMessage();
println!("{}" , msg);
}
