struct Player {
    name: String,
    hp: i32,
}

impl Player {
    fn new(name: String) -> Self {
        Self { name, hp: 100 }
    }

    fn Damage(&mut self, amount: i32) {
        if amount > 0 {
            self.hp -= amount;
            if self.hp < 0 {
                self.hp = 0;
            }
        }
    }

    fn heal(&mut self, amount: i32) {
        if amount > 0 {
            self.hp += amount;
            if self.hp > 100 {
                self.hp = 100;
            }
        }
    }

    fn is_dead(&self) -> bool {
        self.hp <= 0
    }
}

fn main() {
    let mut p = Player::new("Hero".to_string());

    p.Damage(30);
    println!("After 30 damage . Dead {}", p.is_dead());

    p.Damage(80);
    println!("After more damame / Dead {}", p.is_dead());

    p.Damage(50);
    println!("After more damage , dead ? {}", p.is_dead());
}
