fn main() {
    let arr = [1, 4, 23, 46, 87, 89, 99];
    let target = 46;
    let search = binery_search(&arr, target);
    println!("{:?}", search);
}

fn binery_search(arr: &[i32], target: i32) -> Option<usize> {
    let (mut left, mut right) = (0, arr.len());
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}
