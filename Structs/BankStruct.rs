struct BankAccount {
    balance: i64,
}

impl BankAccount {
    fn new() -> Self {
        Self { balance: 0 }
    }

    fn deposit(&mut self, amount: i64) {
        if amount > 0 {
            self.balance += amount;
        }
    }

    fn withdraw(&mut self, amount: i64) {
        if amount > 0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    fn balance(&self) -> i64 {
        self.balance
    }
}

fn main() {
    let mut acc = BankAccount::new();
    acc.deposit(1000);
    acc.withdraw(300);
    println!("Balance is = {}", acc.balance());
}
