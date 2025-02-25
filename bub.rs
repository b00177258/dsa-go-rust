fn bubble_sort(arr: &mut Vec<i32>) -> &mut Vec<i32> {
    for i in 0..arr.len() -1 {
        for j in 0..arr.len() -1{
            if arr[j] > arr[j+1]{
                arr.swap(j, j+1);
            }
        }

    }
    arr
}

fn main() {
    let mut arr = vec![23,1,12,11,10,3];
    let m = bubble_sort(&mut arr);
    println!("{:?}", m);
}
