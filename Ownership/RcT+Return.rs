use std::rc::Rc;

fn main() {
    let Original = Rc::new(String::from("Hello"));
    let another_owner = share_rc(&Original);
    println!(" count :{}", Rc::strong_count(&Original));
}

fn share_rc(r: &Rc<String>) -> Rc<String> {
    Rc::clone(r)
c}
