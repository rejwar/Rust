struct Team {
    name: String,
    members: Vec<String>,
}

fn main() {
    let team = Team {
        name: String::from("Developers"),
        members: vec![String::from("Alice"), String::from("Bob")],
    };
    println!("Team: {}, Members: {:?}", team.name, team.members);
}
