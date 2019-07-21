#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use super::{merge_sort, merge_sort_cmp};

    fn compare_f64(a: &f64, b: &f64) -> Ordering {
        if a == b {
            Ordering::Equal
        } else if a < b {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    #[test]
    fn integers() {
        let result = merge_sort(&vec![9,8,7,6,5,4,3,2,1], |a, b| a.cmp(&b));
        assert_eq!(vec![1,2,3,4,5,6,7,8,9], result);
        let result = merge_sort(&vec![8,7,6,5,4,3,2,1], |a, b| a.cmp(&b));
        assert_eq!(vec![1,2,3,4,5,6,7,8], result);
        let result = merge_sort_cmp(&vec![8,7,6,5,4,3,2,1]);
        assert_eq!(vec![1,2,3,4,5,6,7,8], result);
    }

    #[test]
    fn floats() {
        let result = merge_sort(&vec![9.0,8.0,7.0,6.0,5.0,4.0,3.0,2.0,1.0], |a, b| compare_f64(&a, &b));
        assert_eq!(vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0], result);
        let result = merge_sort(&vec![8.0,7.0,6.0,5.0,4.0,3.0,2.0,1.0], |a, b| compare_f64(&a, &b));
        assert_eq!(vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0], result);
    }

    #[test]
    fn single_element() {
        assert_eq!(vec![0], merge_sort(&vec![0], |_, _| Ordering::Equal))
    }

    #[test]
    fn structs() {
        #[derive(Debug, Clone, PartialEq)]
        struct TestStruct {
            pub f: f64
        }
        assert_eq!(
            vec![TestStruct{f: -0.5}, TestStruct{f: 0.5}],
            merge_sort(&vec![TestStruct{f: 0.5}, TestStruct{f: -0.5}], |a, b| compare_f64(&a.f, &b.f))
        );
    }

    #[test]
    fn test10k() {
        let base: u32 = 100;
        let len = base * base;
        let mut actual: Vec<u32> = Vec::with_capacity(len as usize);
        let mut expected: Vec<u32> = Vec::with_capacity(len as usize);
        for i in 0..len {
            actual.push(i / base);
            expected.push(99 - i % base);
        }
        assert_eq!(actual, merge_sort(&expected, |a, b| a.cmp(&b)));
    }
}

use std::rc::Rc;
use std::cmp::{Ordering, Ord};

pub fn merge_sort<T, F>(input: &Vec<T>, compare: F) -> Vec<T>
where
    T: Clone,
    F: Fn(T, T) -> Ordering
{
    let compare = Rc::new(Box::new(compare));
    merge_sort_internal(input, compare)
}

pub fn merge_sort_cmp<T: Clone + Ord>(input: &Vec<T>) -> Vec<T> {
    let compare = Rc::new(Box::new(|a: T, b: T| a.cmp(&b)));
    merge_sort_internal(input, compare)
}

fn merge_sort_internal<T, F>(input: &Vec<T>, compare: Rc<Box<F>>) -> Vec<T>
where
    T: Clone,
    F: Fn(T, T) -> Ordering
{
    let len = input.len();
    if len == 1 {
        return vec![input[0].clone()];
    }
    let half_len = len / 2;
    let left = {
        let mut left = Vec::with_capacity(half_len);
        left.extend_from_slice(&input[..half_len]);
        merge_sort_internal(&left, compare.clone())
    };
    let right = {
        let mut right = Vec::with_capacity(len - half_len);
        right.extend_from_slice(&input[half_len..]);
        merge_sort_internal(&right, compare.clone())
    };
    merge(left, right, compare)
}

fn merge<T, F>(left: Vec<T>, right: Vec<T>, compare: Rc<Box<F>>) -> Vec<T>
where
    T: Clone,
    F: Fn(T, T) -> Ordering
{
    let left_len = left.len();
    let right_len = right.len();
    let mut result = Vec::with_capacity(left_len + right_len);
    let mut i = 0;
    let mut j = 0;
    while i < left_len && j < right_len {
        use Ordering::{Equal, Greater, Less};
        match compare(left[i].clone(), right[j].clone()) {
            Equal => {
                result.push(left[i].clone());
                result.push(right[j].clone());
                i += 1;
                j += 1;
            },
            Greater => {
                result.push(right[j].clone());
                j += 1;
            },
            Less => {
                result.push(left[i].clone());
                i += 1;
            }
        }
        if i >= left_len {
            result.extend_from_slice(&right[j..]);
            break;
        }
        if j >= right_len {
            result.extend_from_slice(&left[i..]);
            break;
        }
    }
    result
}
