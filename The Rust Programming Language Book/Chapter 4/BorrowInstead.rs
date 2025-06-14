fn main() {
    let Words = vec![String::from("one"), String::from("two")];

    let Uppercase: Vec<String> = Words
        .into_iter() // Takes ownership
        .map(|Word| Word.to_uppercase())
        .collect();

    println!("{:?}", Uppercase);
}
