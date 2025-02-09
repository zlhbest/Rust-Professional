/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

/// quick sorting
fn sort<T: PartialOrd + Clone>(array: &mut [T]) {
    bubble_sort(array);
}
///bubble sorting 完成已提交
fn bubble_sort<T: PartialOrd + Clone>(array: &mut [T]) {
    for i in 0..array.len() - 1 {
        for j in i + 1..array.len() {
            if array[i] > array[j] {
                let temp = array[i].clone();
                array[i] = array[j].clone();
                array[j] = temp;
            }
        }
    }
}
/// quick 快排
fn quick_sort<T: PartialOrd + Clone>(array: &mut [T]) {
    let (left, right) = (0, array.len() - 1);
    while left < right {
        let mid = left + (right - left) / 2;
        if array[left] > array[mid] {}
    }
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
