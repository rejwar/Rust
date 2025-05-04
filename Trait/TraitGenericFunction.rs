trait Printable {
    fn print(&self);
}

struct Report;
struct Invoice;

impl Printable for Report {
    fn print(&self) {
        println!("Printing report...");
    }
}

impl Printable for Invoice {
    fn print(&self) {
        println!("Printing invoice...");
    }
}

fn print_item<T: Printable>(item: T) {
    item.print();
}

fn main() {
    let doc1 = Report;
    let doc2 = Invoice;

    print_item(doc1);
    print_item(doc2);
}
