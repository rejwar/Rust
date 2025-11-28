struct Wallet {
    money: u32,
}

impl Wallet {
    // ১. নতুন ওয়ালেট তৈরি করুন (Associated Function)
    // হিন্ট: self থাকবে না, Wallet রিটার্ন করবে
    fn new(initial_amount: u32) -> ________ {
        Wallet { money: initial_amount }
    }

    // ২. ব্যালেন্স চেক করুন (Immutable Method)
    // হিন্ট: শুধু দেখব, পরিবর্তন করব না
    fn report(________) {
        println!("Balance: {} Taka", self.money);
    }

    // ৩. টাকা খরচ করুন (Mutable Method)
    // হিন্ট: money এর মান কমবে (পরিবর্তন হবে)
    fn spend(________, amount: u32) {
        if self.money >= amount {
            self.money -= amount;
            println!("Spent {} Taka", amount);
        } else {
            println!("Not enough money!");
        }
    }
}

fn main() {
    let mut my_wallet = Wallet::new(500);
    my_wallet.report();
    my_wallet.spend(100);
    my_wallet.report();
}