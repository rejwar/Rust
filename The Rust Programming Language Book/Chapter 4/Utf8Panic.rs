fn main() {
    let bangla = "বাংলা"; // 3-byte per character

    // let slice = &bangla[0..2]; // ❌ Panic: not valid UTF-8 boundary
    let safe = &bangla[0..3];     // ✅ Valid slice for first character

    println!("{}", safe);
}
