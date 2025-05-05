use std::collections::HashMap;

fn Explore(Graph: &HashMap<i32, Vec<i32>>, Node: i32, Visited: &mut HashMap<i32, bool>) {
    Visited.insert(Node, true);
    println!("Component Node: {}", Node);

    if let Some(Neighbors) = Graph.get(&Node) {
        for &Neighbor in Neighbors {
            if !Visited.contains_key(&Neighbor) {
                Explore(Graph, Neighbor, Visited);
            }
        }
    }
}

fn ConnectedComponents(Graph: &HashMap<i32, Vec<i32>>) {
    let mut Visited = HashMap::new();
    for &Node in Graph.keys() {
        if !Visited.contains_key(&Node) {
            println!("New Component Found:");
            Explore(Graph, Node, &mut Visited);
        }
    }
}

fn main() {
    let mut Graph = HashMap::new();
    Graph.insert(1, vec![2]);
    Graph.insert(2, vec![1]);
    Graph.insert(3, vec![4]);
    Graph.insert(4, vec![3]);
    Graph.insert(5, vec![]);

    ConnectedComponents(&Graph);
}
