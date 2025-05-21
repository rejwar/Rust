fn LargestRectangleInHistogram(heights: &[i32]) -> i32 {
    let mut stack = vec![];
    let mut max_area = 0;
    let mut heights = heights.to_vec();
    heights.push(0);

    for (i, &h) in heights.iter().enumerate() {
        while let Some(&last) = stack.last() {
            if heights[last] >= h {
                stack.pop();
                let width = if stack.is_empty() { i } else { i - stack.last().unwrap() - 1 };
                max_area = max_area.max(heights[last] * width as i32);
            } else {
                break;
            }
        }
        stack.push(i);
    }

    max_area
}

fn main() {
    let heights = [2, 1, 5, 6, 2, 3];
    println!("Largest Rectangle Area: {}", LargestRectangleInHistogram(&heights));
}
