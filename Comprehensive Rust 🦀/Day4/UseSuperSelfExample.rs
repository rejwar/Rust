mod outer {
    pub fn outer_fn() {
        println!("Called outer_fn");
    }

    pub mod inner {
        pub fn inner_fn() {
            println!("Called inner_fn");
        }

        pub fn call_all() {
            // ✅ self refers to current module: inner
            self::inner_fn();

            // ✅ super refers to parent module: outer
            super::outer_fn();

            // ✅ use brings item into scope
            use super::outer_fn;
            outer_fn(); // now accessible without full path
        }
    }
}

fn main() {
    outer::inner::call_all();
}
