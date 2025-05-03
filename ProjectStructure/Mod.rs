mod models;

fn main() {
    let Individual = models::CreatePerson();
    match Individual.Position {
        models::Role::Admin => println!("{} is an Admin!", Individual.Name),
        models::Role::User => println!("{} is a User!", Individual.Name),
    }
}
