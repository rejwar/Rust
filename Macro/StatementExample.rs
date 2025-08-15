macro_rules! WrapStmt {
    ($s: stmt) => {
        {
        println!("Before statement ");
        $s
        println!("After statement ");
    }
};
}

fn main() {
    WrapStmt!(let x = 5);
}