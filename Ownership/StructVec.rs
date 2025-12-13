// FileName: StructVec.rs
struct Group {
    members: Vec<String>,
}
fn main() {
    let g = Group {
        members: vec![String::from("A")],
    };
    let m = g.members; // Moves Vec
}
