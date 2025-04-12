struct Account {
    IsActive: bool,
}

impl Account {
    fn CheckStatus(&self) -> bool {
        self.IsActive
    }
}

fn main() {
    let UserAccount: Account = Account { IsActive: true };
    let Status: bool = UserAccount.CheckStatus();
    println!("Account Active: {}", Status);
}
