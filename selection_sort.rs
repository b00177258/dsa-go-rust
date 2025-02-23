fn selction_sort( arr: &mut Vec<i32>) -> Vec<i32> {
    for i in 0..arr.len() -1{
       let  mut min_index  = i;
        for j in i+1 ..arr.len(){
            if arr[min_index] > arr[j] {
                min_index = j;
            }
        }
        arr.swap(min_index, i);
    }
    return arr.to_vec()
}


fn main() {
    let mut arr : Vec<i32> = vec![11,2,12,1,23,4,24,5];
    println!("{:?}", selction_sort(&mut arr));
}
