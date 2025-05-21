fn MergeOverlappingIntervals(intervals: &mut Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    intervals.sort_unstable_by_key(|&(start, _)| start);
    let mut merged = vec![intervals[0]];

    for &(start, end) in intervals.iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if start <= last.1 {
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end));
        }
    }

    merged
}

fn main() {
    let mut intervals = vec![(1, 3), (2, 6), (8, 10), (15, 18)];
    println!("Merged Intervals: {:?}", MergeOverlappingIntervals(&mut intervals));
}
