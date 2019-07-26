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
    merge_sort_parallel_internal(input.to_owned(), threads)
}

pub fn merge_sort<T: Clone + PartialOrd>(input: &[T]) -> MergeResult<Vec<T>> {
    let mut input = input.to_owned();
    let mut temp = vec![input[0].clone(); input.len()];
    merge_sort_internal(&mut input, &mut temp)?;
    Ok(input)
}

pub fn merge_sort_parallel_internal<T: 'static + Send + Clone + PartialOrd>(
    input: Vec<T>,
    threads: usize,
) -> MergeResult<Vec<T>> {
    if threads <= 1 {
        let mut input = input;
        let mut temp = vec![input[0].clone(); input.len()];
        merge_sort_internal(&mut input, &mut temp)?;
        return Ok(input);
    }
    let len = input.len();
    if len == 1 {
        return Ok(input);
    }
    let half_len = len / 2;
    let handler = {
        let input = input[..half_len].to_owned();
        let threads = threads / 2;
        thread::spawn(move || merge_sort_parallel_internal(input, threads))
    };
    let mut right = match merge_sort_parallel_internal(input[half_len..].to_owned(), threads / 2) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    let mut left = match handler.join() {
        Ok(left_result) => left_result?,
        Err(_) => return Err("Recive parallel data error"),
    };
    let mut result: Vec<T> = Vec::with_capacity(len);
    result.append(&mut left);
    result.append(&mut right);
    let mut temp = vec![result[0].clone(); result.len()];
    merge(&mut result, &mut temp)?;
    Ok(result)
}

pub fn merge_sort_internal<T: Clone + PartialOrd>(
    input: &mut [T],
    temp: &mut [T],
) -> MergeResult<()> {
    let len = input.len();
    if len == 1 {
        return Ok(());
    }
    let half_len = len / 2;
    let left = &mut input[..half_len];
    if let Err(e) = merge_sort_internal(left, temp) {
        return Err(e);
    }
    let right = &mut input[half_len..];
    if let Err(e) = merge_sort_internal(right, temp) {
        return Err(e);
    }
    merge(input, temp)
}

fn merge<'a, T: Clone + PartialOrd>(input: &mut [T], temp: &mut [T]) -> MergeResult<()> {
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
                    set_tmp(&mut p, temp, &input[i]);
                    set_tmp(&mut p, temp, &input[j]);
                    i += 1;
                    j += 1;
                }
                Greater => {
                    set_tmp(&mut p, temp, &input[j]);
                    j += 1;
                }
                Less => {
                    set_tmp(&mut p, temp, &input[i]);
                    i += 1;
                }
            },
            None => return Err("Can't compare"),
        }
    }
    if i < half_len {
        for k in i..half_len {
            set_tmp(&mut p, temp, &input[k]);
        }
    } else if j < len {
        for k in j..len {
            set_tmp(&mut p, temp, &input[k]);
        }
    }
    for i in 0..len {
        input[i] = temp[i].clone();
    }
    Ok(())
}

fn set_tmp<'a, T: Clone>(index: &mut usize, temp: &mut [T], value: &T) {
    temp[*index] = value.clone();
    *index += 1;
}
