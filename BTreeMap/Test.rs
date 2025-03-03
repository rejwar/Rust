fn main() {
    let mut btree = Btree::new(2);
    btree.insert(10);
    btree.insert(20);

    btree.transverse();
    
}