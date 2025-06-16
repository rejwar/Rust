// Question: How can we use Box smart pointers to store struct data on heap?

struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn UseBoxedStruct() {
    let root = TreeNode {
        value: 1,
        left: Some(Box::new(TreeNode {
            value: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            value: 3,
            left: None,
            right: None,
        })),
    };

    println!("Root value: {}", root.value);
}
