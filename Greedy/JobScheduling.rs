fn JobScheduling(mut Jobs: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    Jobs.sort_by(|a, b| b.1.cmp(&a.1)); // ✅ Sorting by profit

    let mut ScheduledJobs = Vec::new();
    let mut TimeSlot = 0;

    for &(Deadline, Profit) in &Jobs {
        if TimeSlot < Deadline {
            ScheduledJobs.push((Deadline, Profit));
            TimeSlot += 1;
        }
    }

    ScheduledJobs
}

fn main() {
    let Jobs = vec![(2, 100), (1, 50), (2, 200), (1, 20)];
    println!("Scheduled Jobs: {:?}", JobScheduling(Jobs)); // ✅ Greedy Applied
}
