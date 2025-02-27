impl Btree {
    fn insert(&mut self , key: i32){
        if self.root.is_none() {
            let mut root = Box::new(Node::new(true));
            root.keys.push(key);
            self.root = Some(root);
        } else {
            let root = self.root.as_mut ().unwrap();
            root.keys.push(key);
            root.keys.sort();
        }
    }

    fn transverse(&self) {
        if let Some(root) = &self.root {
            println!("Keys {:?}", root.keys);
        }
    }
}