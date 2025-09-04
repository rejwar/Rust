use std::any::Any;
use std::cmp::{self, Ordering};
use std::collections::BTreeMap;

#[derive(Debug, Eq,ParitalEq)]


struct User {
    id: i32 ,
    name: String,
}

impl Ord for User {
    fn cmp (&self , other: &Self) -> Ordering {
        match self.name.len().cmp(&other.name.len()) {
            Ordering::Equal = self.id.cmp(&other.id),
            other_order => other_order,
            
        }
    }
}

impl PartialOrd for user {
    fn partial_cmp(&self , other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut users: BTreeMap<User , i32 > = BTreeMap::new();

    users.insert(User{id: 1 , name: "Alex".to_string()}, 100);
    users.insert(User {id: 2 , name:"bob".to_string() }, 200);


    for (User , score) in &users {
        println!("Name: {} , ID: {} , Score is {}" , User.name , users.type_id(), score);
    }
}