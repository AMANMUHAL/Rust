fn first_occurrence_index(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    let mut result: Option<usize> = None;

    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] == target {
            result = Some(mid);
            high = mid - 1; // Move to the left to find the first occurrence
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    result
}

fn main() {
    let arr = [1, 2, 2, 3, 3, 3, 4, 5, 5, 6];
    let target = 3;

    if let Some(index) = first_occurrence_index(&arr, target) {
        println!("The first occurrence of {} is at index {}", target, index);
    } else {
        println!("{} is not found in the array", target);
    }
}
