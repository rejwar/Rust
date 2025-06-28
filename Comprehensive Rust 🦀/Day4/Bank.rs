pub mod bank {
    pub struct BankAccount {
        account_number: String, // প্রাইভেট ফিল্ড
        balance: f64,        // প্রাইভেট ফিল্ড
    }

    impl BankAccount {
        pub fn new(account: String) -> BankAccount {
            BankAccount {
                account_number: account,
                balance: 0.0,
            }
        }

        pub fn deposit(&mut self, amount: f64) {
            if amount > 0.0 {
                self.balance += amount;
                println!("Account {} ব্যালেন্স: {}", self.account_number, self.balance);
            } else {
                println!("ডিপোজিট করার জন্য পরিমাণটি ধনাত্মক হতে হবে।");
            }
        }

        pub fn withdraw(&mut self, amount: f64) {
            if amount > 0.0 && amount <= self.balance {
                self.balance -= amount;
                println!("Account {} ব্যালেন্স: {}", self.account_number, self.balance);
            } else {
                println!("অপর্যাপ্ত ব্যালেন্স অথবা অকার্যকর পরিমাণ।");
            }
        }

        pub fn get_balance(&self) -> f64 {
            self.balance
        }
    }
}

fn main() {
    let mut my_account = bank::BankAccount::new("12345".to_string());
    my_account.deposit(100.0);
    my_account.withdraw(30.0);
    println!("বর্তমান ব্যালেন্স: {}", my_account.get_balance());

    // my_account.balance = 1000.0; // এটি কাজ করবে না কারণ 'balance' ফিল্ড প্রাইভেট
}
