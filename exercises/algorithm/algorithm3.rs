/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM NOT DONE

fn sort<T>(array: &mut [T])
where
    T: Ord,
{
    let right = if array.len() == 0 {
        return;
    } else {
        array.len() - 1
    };
    let left = 0;
    if left >= right {
        return;
    }
    let mut low = left;
    let mut high = right;
    while low < high {
        while low < high && array[high] >= array[left] {
            high -= 1;
        }
        while low < high && array[low] <= array[left] {
            low += 1;
        }
        array.swap(low, high);
    }
    array.swap(low, left);
    sort(&mut array[0..low + 1]);
    sort(&mut array[low + 1..]);
}

fn main() {
    let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
    sort(&mut vec);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
