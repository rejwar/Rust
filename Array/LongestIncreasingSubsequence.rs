fn LongestIncreasingSubsequence(arr: &[i32]) -> Vec<i32> {
    let mut lis = vec![vec![]; arr.len()];
    lis[0].push(arr[0]);

    for i in 1..arr.len() {
        for j in 0..i {
            if arr[j] < arr[i] && lis[j].len() + 1 > lis[i].len() {
                lis[i] = lis[j].clone();
            }
        }
        lis[i].push(arr[i]);
    }

    lis.into_iter().max_by_key(|x| x.len()).unwrap()
}

fn main() {
    let arr = [10, 22, 9, 33, 21, 50, 41, 60];
    println!("Longest Increasing Subsequence: {:?}", LongestIncreasingSubsequence(&arr));
}
