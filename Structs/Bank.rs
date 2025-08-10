#[derive(Debug)]

struct BankAccount{
    owner: String,
    balance: i64,
}

impl BankAccount {
    fn deposit (&mut self , amount : i64 ) {
        self.balance += amount;
    }
}

fn main() {
    let mut a = BankAccount {owner : "Sara".to_string(), balance: 0};
    a.deposit(500);
    println!("{:?}", a);
}