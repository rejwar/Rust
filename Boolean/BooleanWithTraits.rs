trait Auth {
    fn HasAccess(&self) -> bool;
}

struct Admin {
    IsSuperAdmin: bool,
}

impl Auth for Admin {
    fn HasAccess(&self) -> bool {
        self.IsSuperAdmin
    }
}

fn main() {
    let AdminUser: Admin = Admin { IsSuperAdmin: true };
    println!("Admin Has Access: {}", AdminUser.HasAccess());
}
