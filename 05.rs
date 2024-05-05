fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        // If the length is even, return the average of the two middle elements
        let mid_index = len / 2;
        (arr[mid_index - 1] + arr[mid_index]) as f64 / 2.0
    } else {
        // If the length is odd, return the middle element
        arr[len / 2] as f64
    }
}

fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 5, 6];

    println!("Median of arr1: {}", median(&arr1));
    println!("Median of arr2: {}", median(&arr2));
}
