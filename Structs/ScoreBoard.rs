struct ScoreBoard {
    home: i32,
    away: i32,
}

impl ScoreBoard {
    fn new() -> Self {
        Self { home: 0, away: 0 }
    }

    fn ScoreBoard(&mut self, points: i32) {
        if points > 0 {
            self.home += points;
        }
    }

    fn ScoreHome(&mut self, points: i32) {
        if points > 0 {
            self.home += points;
        }
    }

    fn ScoreAway(&mut self, points: i32) {
        if points > 0 {
            self.away += points;
        }
    }
    fn Reset(&mut self) {
        self.home = 0;

        self.away = 0;
    }

    fn total(&self) -> i32 {
        self.home + self.away
    }
}

fn main() {
    let mut sb = ScoreBoard::new();
    sb.ScoreHome(3);
    sb.ScoreAway(7);
    println!("Total = {}", sb.total());

    sb.Reset();
    println!("After result {}", sb.total());
}
