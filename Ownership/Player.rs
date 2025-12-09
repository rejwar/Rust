fn main() {
    let players = vec![String::from("Messi"), String::from("Ronaldo")];

    for p in players.into_iter() {
        println!(" Playing {}", p);
    }
}
