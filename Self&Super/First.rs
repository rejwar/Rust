//
// main.rs

mod parent {
    pub fn msg_from_parent() {
        println!("এটা parent থেকে!");
    }

    pub mod child {
        pub fn msg_from_child() {
            println!("এটা child থেকে!");
        }

        pub fn demo() {
            // self keyword: নিজের module-এর function কল
            self::msg_from_child();

            // super keyword: parent module-এর function কল
            super::msg_from_parent();
        }
    }
}

fn main() {
    parent::child::demo();
}
