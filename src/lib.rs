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
    /*
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
    } */
}

use std::cmp::{Ordering, PartialOrd};
// use std::marker::Send;

pub type MergeResult<T> = Result<T, &'static str>;
/*
pub fn merge_sort_parallel<T: 'static + Clone + PartialOrd + Send>(
    input: &[T],
    threads: u8,
) -> MergeResult<T> {
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
        Err(e) => return Err(e),
    };
    let left = match rx.recv() {
        Ok(left_result) => match left_result {
            Ok(r) => r,
            Err(e) => return Err(e),
        },
        Err(_) => return Err("Recive parallel data error"),
    };
    merge(left, right)
}
*/

pub fn merge_sort<T: Clone + PartialOrd>(input: &[T]) -> MergeResult<Vec<T>> {
    let mut input = input.to_owned();
    let mut temp = vec![input[0].clone(); input.len()];
    merge_sort_internal(&mut input, &mut temp)?;
    Ok(input)
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
