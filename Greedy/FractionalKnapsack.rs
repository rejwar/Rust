fn FractionalKnapsack(mut Items: Vec<(i32, i32)>, Capacity: i32) -> f64 {
    Items.sort_by(|a, b| (b.0 as f64 / b.1 as f64).partial_cmp(&(a.0 as f64 / a.1 as f64)).unwrap());

    let mut TotalValue = 0.0;
    let mut RemainingCapacity = Capacity;

    for &(Value, Weight) in &Items {
        if Weight <= RemainingCapacity {
            TotalValue += Value as f64;
            RemainingCapacity -= Weight;
        } else {
            TotalValue += Value as f64 * (RemainingCapacity as f64 / Weight as f64);
            break;
        }
    }

    TotalValue
}

fn main() {
    let Items = vec![(60, 10), (100, 20), (120, 30)];
    let Capacity = 50;

    println!("Maximum Value: {:.2}", FractionalKnapsack(Items, Capacity)); // ✅ Greedy Applied
}
