// A simple struct to represent a user.
struct User {
    username: String,
    active: bool,
}

impl User {
    // This is an **Associated Function** (often called a constructor).
    // It does NOT take `self` as a parameter.
    // It is called on the type: `User::build_guest(...)`.
    pub fn build_guest(username: &str) -> User {
        User {
            username: String::from(username),
            active: false, // Guests are inactive by default
        }
    }

    // This is another Associated Function that creates a default instance.
    pub fn default_admin() -> User {
        User {
            username: String::from("admin_user"),
            active: true,
        }
    }

    // This is a **Method** because it takes `&self`.
    // It is called on an instance: `my_user.get_status()`.
    pub fn get_status(&self) -> &str {
        if self.active {
            "Active"
        } else {
            "Inactive"
        }
    }
}

fn main() {
    // Call the associated function using the Type name and `::`
    let guest = **User::build_guest * *("temporary_visitor");

    // Call another associated function
    let admin = **User::default_admin * *();

    // Call a method on the instances
    println!("{}'s status: {}", guest.username, guest.get_status());
    println!("{}'s status: {}", admin.username, admin.get_status());
}
