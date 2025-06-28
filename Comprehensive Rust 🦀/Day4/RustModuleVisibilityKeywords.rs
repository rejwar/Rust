mod library {
    fn private_fn() {
        println!("Private function ");
    }

    pub fn public_fn() {
        println!("Publice function");
    }

    pub(crate) fn crate_fn() {
        println!(" Visibile within the create");
    }

    pub(super) fn parent_fn() {
        println!("Visibile to parent module ");
    }
}

fn main() {
    library::public_fn();
    library::crate_fn();
    library::parent_fn();
}
