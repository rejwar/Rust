use std::collections::VecDeque;
use std::collections::HashMap;

fn Bfs(Graph: &HashMap<i32, Vec<i32>>, Start: i32) {
    let mut Queue = VecDeque::new();
    let mut Visited = HashMap::new();
    
    Queue.push_back(Start);
    Visited.insert(Start, true);

    while let Some(Node) = Queue.pop_front() {
        println!("Visited: {}", Node);
        if let Some(Neighbors) = Graph.get(&Node) {
            for &Neighbor in Neighbors {
                if !Visited.contains_key(&Neighbor) {
                    Queue.push_back(Neighbor);
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

    Bfs(&Graph, 1);
}
