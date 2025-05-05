use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
struct HuffmanNode {
    Frequency: i32,
    Left: Option<Box<HuffmanNode>>,
    Right: Option<Box<HuffmanNode>>,
}

impl Ord for HuffmanNode {
    fn cmp(&self, Other: &Self) -> Ordering {
        Other.Frequency.cmp(&self.Frequency)
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, Other: &Self) -> Option<Ordering> {
        Some(self.cmp(Other))
    }
}

fn HuffmanCoding(Frequencies: &HashMap<char, i32>) {
    let mut Heap = BinaryHeap::new();
    
    for (&Char, &Freq) in Frequencies {
        Heap.push(HuffmanNode { Frequency: Freq, Left: None, Right: None });
    }

    while Heap.len() > 1 {
        let LeftNode = Heap.pop().unwrap();
        let RightNode = Heap.pop().unwrap();

        let NewNode = HuffmanNode {
            Frequency: LeftNode.Frequency + RightNode.Frequency,
            Left: Some(Box::new(LeftNode)),
            Right: Some(Box::new(RightNode)),
        };

        Heap.push(NewNode);
    }

    println!("Huffman Tree Built Successfully!");
}

fn main() {
    let mut Frequencies = HashMap::new();
    Frequencies.insert('A', 5);
    Frequencies.insert('B', 9);
    Frequencies.insert('C', 12);
    Frequencies.insert('D', 13);
    Frequencies.insert('E', 16);
    Frequencies.insert('F', 45);

    HuffmanCoding(&Frequencies);
}
