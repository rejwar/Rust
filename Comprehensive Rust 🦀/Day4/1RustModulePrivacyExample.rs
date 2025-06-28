// nested module
pub mod player {
    pub mod sprite {
        pub fn create() {
            println!("called player::sprite::create");
        }
    }
}

fn main() {
    // call public function create from sprite module which is inside player module 
    player::sprite::create();
}
