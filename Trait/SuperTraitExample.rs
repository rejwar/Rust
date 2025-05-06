trait Readable {
    fn Read(&self);
}

trait Writable: Readable {
    fn Write(&self);
}

struct File;

impl Readable for File {
    fn Read(&self) {
        println!("Reading File...");
    }
}

impl Writable for File {
    fn Write(&self) {
        println!("Writing to File...");
    }
}

fn main() {
    let Doc = File;
    Doc.Read();
    Doc.Write();
}
