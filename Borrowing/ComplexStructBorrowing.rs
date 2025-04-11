struct Team {
    Name: String,
    Score: u32,
}

fn DisplayScore(Team: &Team) {
    println!("Team: {} | Score: {}", Team.Name, Team.Score);
}

fn main() {
    let MyTeam: Team = Team {
        Name: String::from("Warriors"),
        Score: 95,
    };
    DisplayScore(&MyTeam);
}
