use std::cmp::Ord;

fn bubble_sort<T: Ord>(arr: &mut [T]) -> &[T] {
    let mut stop = arr.len() - 1;
    while stop > 0 {
        for i in 0..stop {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
            }
        }
        stop -= 1;
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = [9, 3, 7, 4, 69, 420, 42];
        assert_eq!(bubble_sort(&mut arr), [3, 4, 7, 9, 42, 69, 420]);
    }
}
