#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integers() {
        assert_eq!(
            Ok(vec![1,2,3,4,5,6,7,8,9]),
            merge_sort(&vec![9,8,7,6,5,4,3,2,1])
        );
        assert_eq!(
            Ok(vec![1,2,3,4,5,6,7,8]),
            merge_sort(&vec![8,7,6,5,4,3,2,1])
        );
    }

    #[test]
    fn floats() {
        assert_eq!(Err("Can't compare"), merge_sort(&vec![std::f64::NAN, std::f64::NAN]));
        assert_eq!(
            Ok(vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0]),
            merge_sort(&vec![9.0,8.0,7.0,6.0,5.0,4.0,3.0,2.0,1.0])
        );
        assert_eq!(
            Ok(vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0]),
            merge_sort(&vec![8.0,7.0,6.0,5.0,4.0,3.0,2.0,1.0])
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(Ok(vec![0]), merge_sort(&vec![0]));
    }

    #[test]
    fn structs() {
        #[derive(Debug, Clone, PartialEq, PartialOrd)]
        struct TestStruct(f64);
        assert_eq!(
            Ok(vec![TestStruct(-0.5), TestStruct(0.5)]),
            merge_sort(&vec![TestStruct(0.5), TestStruct(-0.5)])
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
            expected.push((base - 1) - i % base);
        }
        assert_eq!(Ok(actual), merge_sort(&expected));
    }

    #[test]
    fn test10k_parallel() {
        let base: u32 = 100;
        let len = base * base;
        let mut actual: Vec<u32> = Vec::with_capacity(len as usize);
        let mut expected: Vec<u32> = Vec::with_capacity(len as usize);
        for i in 0..len {
            actual.push(i / base);
            expected.push((base - 1) - i % base);
        }
        assert_eq!(Ok(actual), merge_sort_parallel(&expected, 4));
    }
}

use std::cmp::{Ordering, PartialOrd};
use std::marker::Send;

pub type MergeResult<T> = Result<Vec<T>, &'static str>;

pub fn merge_sort_parallel<T: 'static + Clone + PartialOrd + Send>(input: &[T], threads: u8) -> MergeResult<T> {
    if threads <= 1 {
        return merge_sort(input);
    }
    let chunk_len = input.len() / (threads as usize);
    let rx = {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut thread_input = Vec::with_capacity(chunk_len);
        thread_input.extend_from_slice(&input[..chunk_len]);
        std::thread::spawn(move || {
            let input = thread_input;
            tx.send(merge_sort(&input)).unwrap();
        });
        rx
    };
    let right = match merge_sort_parallel(&input[chunk_len..], threads - 1) {
        Ok(r) => r,
        Err(e) => return Err(e)
    };
    let left = match rx.recv() {
        Ok(left_result) => match left_result {
            Ok(r) => r,
            Err(e) => return Err(e)
        },
        Err(_) => return Err("Recive parallel data error")
    };
    merge(left, right)
}

pub fn merge_sort<T: Clone + PartialOrd>(input: &[T]) -> MergeResult<T> {
    let len = input.len();
    if len == 1 {
        return Ok(vec![input[0].clone()]);
    }
    let half_len = len / 2;
    let left = match merge_sort(&input[..half_len]) {
        Ok(r) => r,
        Err(e) => return Err(e)
    };
    let right = match merge_sort(&input[half_len..]) {
        Ok(r) => r,
        Err(e) => return Err(e)
    };
    merge(left, right)
}

fn merge<T: Clone + PartialOrd>(left: Vec<T>, right: Vec<T>) -> MergeResult<T> {
    let left_len = left.len();
    let right_len = right.len();
    let mut result = Vec::with_capacity(left_len + right_len);
    let mut i = 0;
    let mut j = 0;
    while i < left_len && j < right_len {
        use Ordering::{Equal, Greater, Less};
        match left[i].partial_cmp(&right[j]) {
            Some(v) => match v {
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
            },
            None => return Err("Can't compare")
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
    Ok(result)
}
