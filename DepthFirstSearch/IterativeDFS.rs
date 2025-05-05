use std::collections::{HashMap, VecDeque};

fn IterativeDfs(Graph: &HashMap<i32, Vec<i32>>, Start: i32) {
    let mut Stack = VecDeque::new();
    let mut Visited = HashMap::new();

    Stack.push_back(Start);
    Visited.insert(Start, true);

    while let Some(Node) = Stack.pop_back() {
        println!("Visited: {}", Node);
        if let Some(Neighbors) = Graph.get(&Node) {
            for &Neighbor in Neighbors {
                if !Visited.contains_key(&Neighbor) {
                    Stack.push_back(Neighbor);
                    Visited.insert(Neighbor, true);
                }
            }
        }
    }
}

fn main() {
    let mut Graph = HashMap::new();
    Graph.insert(1, vec![2, 3]);
    Graph.insert(2, vec![4]);
    Graph.insert(3, vec![4, 5]);
    Graph.insert(4, vec![]);
    Graph.insert(5, vec![]);

    IterativeDfs(&Graph, 1);
}
