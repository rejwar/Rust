struct  User {
    username: String,
    active: bool,
}

fn main() {
    let user1 = User{
        username:String::from("rustacean"),
        active:true,
    };

    let user2 = User {
        username:String::from ("other"),
        ..user1
    };

    println!("{} {}", user2.username , user2.active);
}
