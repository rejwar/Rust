fn CheckFileAccess(HasPermission: bool, IsFileLocked: bool) -> Result<bool, &'static str> {
    if HasPermission && !IsFileLocked {
        Ok(true) // Access granted
    } else if !HasPermission {
        Err("Permission Denied!")
    } else {
        Err("File is Locked!")
    }
}

fn main() {
    let HasPermission: bool = true;
    let IsFileLocked: bool = false;

    match CheckFileAccess(HasPermission, IsFileLocked) {
        Ok(Access) => println!("Access Granted: {}", Access),
        Err(Error) => println!("Error: {}", Error),
    }
}
