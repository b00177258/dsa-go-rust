fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);

    // Merge sorted halves
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    // Append remaining elements from left
    while i < left.len() {
        result.push(left[i]);
        i += 1;
    }

    // Append remaining elements from right
    while j < right.len() {
        result.push(right[j]);
        j += 1;
    }

    result
}

fn mergesort(arr: &[i32]) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr.to_vec(); // Convert slice to Vec
    }

    let mid = arr.len() / 2;
    let left = mergesort(&arr[..mid]);
    let right = mergesort(&arr[mid..]);

    merge(&left, &right)
}

fn main() {
    let arr = [6, 3, 8, 5, 2];
    let sorted_arr = mergesort(&arr);
    println!("Sorted array: {:?}", sorted_arr);
}
