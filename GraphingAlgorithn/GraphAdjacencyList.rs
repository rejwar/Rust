use std::collections::HashMap;

struct Graph {
    AdjacencyList: HashMap<i32, Vec<i32>>,
}

impl Graph {
    fn New() -> Self {
        Graph { AdjacencyList: HashMap::new() }
    }

    fn AddEdge(&mut self, Src: i32, Dest: i32) {
        self.AdjacencyList.entry(Src).or_insert(Vec::new()).push(Dest);
        self.AdjacencyList.entry(Dest).or_insert(Vec::new()).push(Src);
    }

    fn PrintGraph(&self) {
        for (Node, Neighbors) in &self.AdjacencyList {
            println!("Node {} -> {:?}", Node, Neighbors);
        }
    }
}

fn main() {
    let mut G = Graph::New();
    G.AddEdge(1, 2);
    G.AddEdge(1, 3);
    G.AddEdge(2, 4);
    G.PrintGraph();
}
