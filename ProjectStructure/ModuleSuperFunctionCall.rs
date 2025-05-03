mod outer {
    pub mod inner {
        pub(super) fn secret_function() {
            println!("Accessible from parent module");
        }
    }

    pub fn call_secret() {
        inner::secret_function();
    }
}

fn main() {
    outer::call_secret();
}
