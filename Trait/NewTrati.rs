struct PaperPlane;
struct Bird;

impl Fly for PaperPlane {}

impl Fly for Bird {
    fn Fly(&self) {
        println!("The bird is flying");
    }
}

fn main() {
    let paper_plane = PaperPlane;
    let bird = Bird;

    paper_plane.fly();
    bird.fly();
}
