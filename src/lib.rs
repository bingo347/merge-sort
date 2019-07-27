#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integers() {
        assert_eq!(
            Ok(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            merge_sort(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1])
        );
        assert_eq!(
            Ok(vec![1, 2, 3, 4, 5, 6, 7, 8]),
            merge_sort(&vec![8, 7, 6, 5, 4, 3, 2, 1])
        );
    }

    #[test]
    fn floats() {
        assert_eq!(
            Err("Can't compare"),
            merge_sort(&vec![std::f64::NAN, std::f64::NAN])
        );
        assert_eq!(
            Ok(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]),
            merge_sort(&vec![9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0])
        );
        assert_eq!(
            Ok(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
            merge_sort(&vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0])
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
use std::thread;

pub type MergeResult<T> = Result<T, &'static str>;

pub fn merge_sort_parallel<T: 'static + Send + Clone + PartialOrd>(
    input: &[T],
    threads: usize,
) -> MergeResult<Vec<T>> {
    if threads != 1 && threads % 2 != 0 {
        return Err("Threads count must be power of 2");
    }
    let mut input = input.to_owned();
    merge_sort_parallel_internal(&mut input, threads)?;
    Ok(input)
}

pub fn merge_sort<T: Clone + PartialOrd>(input: &[T]) -> MergeResult<Vec<T>> {
    let mut input = input.to_owned();
    let mut temp = mk_temp(input.len());
    merge_sort_internal(&mut input, &mut temp)?;
    Ok(input)
}

fn merge_sort_parallel_internal<T: 'static + Send + Clone + PartialOrd>(
    input: &mut [T],
    threads: usize,
) -> MergeResult<()> {
    let len = input.len();
    if len == 1 {
        return Ok(());
    }
    if threads <= 1 {
        let mut temp = mk_temp(len);
        merge_sort_internal(input, &mut temp)?;
        return Ok(());
    }
    let mut left = {
        let (left, right) = input.split_at_mut(len / 2);
        let handler_left = thread::spawn({
            let threads = threads / 2;
            let mut left = left.to_owned();
            move || {
                merge_sort_parallel_internal(&mut left, threads)?;
                Ok(left)
            }
        });
        merge_sort_parallel_internal(right, threads / 2)?;
        match handler_left.join() {
            Ok(r) => r?,
            Err(_) => return Err("Recive parallel data error"),
        }
    };
    for i in 0..left.len() {
        swap_elements(&mut left, i, input, i);
    }
    let mut temp = mk_temp(len);
    merge(input, &mut temp)?;
    Ok(())
}

fn merge_sort_internal<T: PartialOrd>(input: &mut [T], temp: &mut [T]) -> MergeResult<()> {
    let len = input.len();
    if len == 1 {
        return Ok(());
    }
    let (left, right) = input.split_at_mut(len / 2);
    merge_sort_internal(left, temp)?;
    merge_sort_internal(right, temp)?;
    merge(input, temp)
}

fn merge<T: PartialOrd>(input: &mut [T], temp: &mut [T]) -> MergeResult<()> {
    use Ordering::{Equal, Greater, Less};
    let len = input.len();
    let half_len = len / 2;
    let mut p = 0;
    let mut i = 0;
    let mut j = half_len;
    while i < half_len && j < len {
        match input[i].partial_cmp(&input[j]) {
            Some(v) => match v {
                Equal => {
                    swap_elements(input, i, temp, p);
                    p += 1;
                    i += 1;
                    swap_elements(input, j, temp, p);
                    p += 1;
                    j += 1;
                }
                Greater => {
                    swap_elements(input, j, temp, p);
                    p += 1;
                    j += 1;
                }
                Less => {
                    swap_elements(input, i, temp, p);
                    p += 1;
                    i += 1;
                }
            },
            None => return Err("Can't compare"),
        }
    }
    if i < half_len {
        for k in i..half_len {
            swap_elements(input, k, temp, p);
            p += 1;
        }
    } else if j < len {
        for k in j..len {
            swap_elements(input, k, temp, p);
            p += 1;
        }
    }
    for i in 0..len {
        swap_elements(temp, i, input, i);
    }
    Ok(())
}

fn swap_elements<T>(from_slice: &mut [T], from_index: usize, to_slice: &mut [T], to_index: usize) {
    unsafe {
        let from_ptr = from_slice.as_mut_ptr().add(from_index);
        let to_ptr = to_slice.as_mut_ptr().add(to_index);
        std::ptr::swap(from_ptr, to_ptr);
    }
}

fn mk_temp<T>(len: usize) -> Vec<T> {
    let mut temp = Vec::with_capacity(len);
    unsafe {
        temp.set_len(len);
    }
    temp
}
