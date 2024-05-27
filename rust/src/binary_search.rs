use std::cmp::{Ord, Ordering};

pub fn binary_search<T: Ord>(haystack: &[T], needle: T) -> Option<usize> {
    let mut left = 0;
    let mut right = haystack.len();
    while left < right {
        let middle = (left + right) / 2;
        match needle.cmp(&haystack[middle]) {
            Ordering::Equal => return Some(middle),
            Ordering::Greater => left = middle + 1,
            Ordering::Less => right = middle,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let haystack = vec![1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];
        assert_eq!(binary_search(&haystack, 69), Some(3));
        assert_eq!(binary_search(&haystack, 1336), None);
        assert_eq!(binary_search(&haystack, 69420), Some(10));
        assert_eq!(binary_search(&haystack, 69421), None);
        assert_eq!(binary_search(&haystack, 1), Some(0));
        assert_eq!(binary_search(&haystack, 0), None);
    }
}
