// Check the sub array is exist or not

fn is_sub_array(origin_array: &[u32], sub_array: &[u32]) -> bool {
    let mut check = false;
    let mut i = 0;
    let mut j = 0;
    while i < origin_array.len() && j < sub_array.len() {
        if origin_array[i] == sub_array[j] {
            i += 1;
            j += 1;
            if j == sub_array.len() {
                check = true;
            }
        } else {
            i = i - j + 1;
            j = 0;
        }
    }
    check
}

fn main() {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10, 12];
    println!("Hello, world!");
    if is_sub_array(&org_arr, &sub_arr) {
        println!("Is sub_array!");
    } else {
        println!("Is not sub_array!");
    }
}
