struct User {

    Name: String,
    Age: u32,
}

enum Account {
    Active (User),
    Suspended(String)
}

fn main() {
    let ActiveUser: Account = Account::Active(User{Name: String::from("Alice ") , Age: 30});
    let SuspendedUser: Account = Account::Suspended(String::from("Policy  Violation"));

    match ActiveUser {
        Account::Active(user) => println!("Active User {} , Age {}" , user.Name , user.Age),
        Account::Suspended(reason) => println!("Suspended Useer. Reason: {} " , reason), 

    }
}
