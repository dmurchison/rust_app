fn main() {
    println!("Hello, world!");
}

fn binary_search(arr: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return mid as i32;
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(binary_search(&arr, 1), 0);
        assert_eq!(binary_search(&arr, 2), 1);
        assert_eq!(binary_search(&arr, 3), 2);
        assert_eq!(binary_search(&arr, 4), 3);
        assert_eq!(binary_search(&arr, 5), 4);
        assert_eq!(binary_search(&arr, 6), 5);
        assert_eq!(binary_search(&arr, 7), 6);
        assert_eq!(binary_search(&arr, 8), 7);
        assert_eq!(binary_search(&arr, 9), -1);
    }
}
