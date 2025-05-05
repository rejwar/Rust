use std::collections::HashMap;

fn Dfs(Graph: &HashMap<i32, Vec<i32>>, Node: i32, Visited: &mut HashMap<i32, bool>) {
    println!("Visited: {}", Node);
    Visited.insert(Node, true);

    if let Some(Neighbors) = Graph.get(&Node) {
        for &Neighbor in Neighbors {
            if !Visited.contains_key(&Neighbor) {
                Dfs(Graph, Neighbor, Visited);
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

    let mut Visited = HashMap::new();
    Dfs(&Graph, 1, &mut Visited);
}
