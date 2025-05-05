use std::collections::HashMap;

fn DetectCycle(Graph: &HashMap<i32, Vec<i32>>, Node: i32, Visited: &mut HashMap<i32, bool>, Parent: i32) -> bool {
    Visited.insert(Node, true);

    if let Some(Neighbors) = Graph.get(&Node) {
        for &Neighbor in Neighbors {
            if !Visited.contains_key(&Neighbor) {
                if DetectCycle(Graph, Neighbor, Visited, Node) {
                    return true;
                }
            } else if Neighbor != Parent {
                return true; // ✅ Cycle Found
            }
        }
    }

    false
}

fn main() {
    let mut Graph = HashMap::new();
    Graph.insert(1, vec![2, 3]);
    Graph.insert(2, vec![1, 4]);
    Graph.insert(3, vec![1, 5]);
    Graph.insert(4, vec![2]);
    Graph.insert(5, vec![3]);

    let mut Visited = HashMap::new();
    let HasCycle = DetectCycle(&Graph, 1, &mut Visited, -1);

    println!("Graph Contains Cycle: {}", HasCycle);
}
