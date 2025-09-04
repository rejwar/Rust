use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive( Debug, Eq , PartialEq)]



struct Score {
    name: String,
    points: i32,
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.points.cmp(&self.points) {
            Ordering::Equal => self.name.cmp(&other.name),
            ord => ord,
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut scores: BTreeMap<Score , i32> = BTreeMap::new();

    scores.insert(Score {name: "Alice ".to_string() , points: 80}, 1);


    for (player, rank) in &scores {
        println!("{} ({} pts ) -> Rank {}", player.name , player.points ,rank);
    }
}