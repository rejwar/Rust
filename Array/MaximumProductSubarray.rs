fn MaximumProductSubarray(arr: &[i32]) -> i32 {
    let mut max_product = arr[0];
    let mut min_product = arr[0];
    let mut result = arr[0];

    for &num in arr.iter().skip(1) {
        if num < 0 {
            std::mem::swap(&mut max_product, &mut min_product);
        }

        max_product = max_product.max(num).max(max_product * num);
        min_product = min_product.min(num).min(min_product * num);

        result = result.max(max_product);
    }

    result
}

fn main() {
    let arr = [2, 3, -2, 4];
    println!("Maximum Product Subarray: {}", MaximumProductSubarray(&arr));
}
