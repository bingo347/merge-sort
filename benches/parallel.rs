#[macro_use]
extern crate bencher;

use bencher::Bencher;

fn bench_single_thread(b: &mut Bencher) {
    let base: u32 = 1000;
    let len = base * base;
    let mut expected: Vec<u32> = Vec::with_capacity(len as usize);
    for i in 0..len {
        expected.push((base - 1) - i % base);
    }
    b.iter(|| merge_sort(&expected))
}

fn bench_parallel_2(b: &mut Bencher) {
    let base: u32 = 1000;
    let len = base * base;
    let mut expected: Vec<u32> = Vec::with_capacity(len as usize);
    for i in 0..len {
        expected.push((base - 1) - i % base);
    }
    b.iter(|| merge_sort_parallel(&expected, 2))
}

fn bench_parallel_4(b: &mut Bencher) {
    let base: u32 = 1000;
    let len = base * base;
    let mut expected: Vec<u32> = Vec::with_capacity(len as usize);
    for i in 0..len {
        expected.push((base - 1) - i % base);
    }
    b.iter(|| merge_sort_parallel(&expected, 4))
}

benchmark_group!(
    benches,
    bench_single_thread,
    bench_parallel_2,
    bench_parallel_4
);
benchmark_main!(benches);


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
