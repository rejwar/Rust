fn ExecuteTwice<F> (action: F) 
where 
F : Fn(),
{
    action () ;
    action ();
}

fn main () {
    ExecuteTwice(|| println!("Running"));
}
