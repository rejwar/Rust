mod bank {
    pub struct Account {
        balance: f64, // private by default
    }

    impl Account {
        pub fn new() -> Self {
            Account { balance: 0.0 }
        }

        pub fn deposit(&mut self, amount: f64) {
            self.balance += amount;
        }

        pub fn balance(&self) -> f64 {
            self.balance
        }
    }
}

fn main() {
    let mut acc = bank::Account::new();
    acc.deposit(100.0);
    println!("Balance: {}", acc.balance());
}
