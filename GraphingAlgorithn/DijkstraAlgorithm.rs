use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
struct Node {
    Cost: i32,
    Vertex: i32,
}

impl Ord for Node {
    fn cmp(&self, Other: &Self) -> Ordering {
        Other.Cost.cmp(&self.Cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, Other: &Self) -> Option<Ordering> {
        Some(self.cmp(Other))
    }
}

fn Dijkstra(Graph: &HashMap<i32, Vec<(i32, i32)>>, Start: i32) {
    let mut Distances = HashMap::new();
    let mut PriorityQueue = BinaryHeap::new();

    PriorityQueue.push(Node { Cost: 0, Vertex: Start });
    Distances.insert(Start, 0);

    while let Some(Node) = PriorityQueue.pop() {
        if let Some(Neighbors) = Graph.get(&Node.Vertex) {
            for &(Neighbor, Weight) in Neighbors {
                let NewCost = Node.Cost + Weight;
                if !Distances.contains_key(&Neighbor) || NewCost < Distances[&Neighbor] {
                    Distances.insert(Neighbor, NewCost);
                    PriorityQueue.push(Node { Cost: NewCost, Vertex: Neighbor });
                }
            }
        }
    }

    println!("Shortest Distances: {:?}", Distances);
}

fn main() {
    let mut Graph = HashMap::new();
    Graph.insert(1, vec![(2, 1), (3, 4)]);
    Graph.insert(2, vec![(3, 2), (4, 5)]);
    Graph.insert(3, vec![(4, 3)]);
    Graph.insert(4, vec![]);

    Dijkstra(&Graph, 1);
}
