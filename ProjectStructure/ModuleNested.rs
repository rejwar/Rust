mod outer {
    pub mod inner {
        pub fn Inside() {
            println!("Inside Inner Moduler");

        }
        pub fn CallSuper() {
            super::ParentFunction();
        }
    }

    pub fn ParentFunction() {
        println!("Called from Super Module");
    }
}

fn main() {
    outer::inner::Inside();
    outer::inner::CallSuper();
}
