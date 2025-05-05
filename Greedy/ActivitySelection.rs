fn ActivitySelection(mut Activities: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    Activities.sort_by(|a, b| a.1.cmp(&b.1)); // ✅ Sorting by finishing time

    let mut SelectedActivities = Vec::new();
    let mut LastFinishTime = 0;

    for &(Start, Finish) in &Activities {
        if Start >= LastFinishTime {
            SelectedActivities.push((Start, Finish));
            LastFinishTime = Finish;
        }
    }

    SelectedActivities
}

fn main() {
    let Activities = vec![(1, 3), (2, 5), (4, 6), (7, 9), (5, 8)];
    println!("Selected Activities: {:?}", ActivitySelection(Activities)); // ✅ Greedy Applied
}
