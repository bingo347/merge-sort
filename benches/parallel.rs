#[macro_use]
extern crate bencher;
extern crate merge_sort;

use bencher::Bencher;
use merge_sort::*;

fn bench_single_thread(b: &mut Bencher) {
    let base: u32 = 200;
    let len = base * base;
    let mut expected: Vec<u32> = Vec::with_capacity(len as usize);
    for i in 0..len {
        expected.push((base - 1) - i % base);
    }
    b.iter(|| merge_sort(&expected).unwrap())
}
/*
fn bench_parallel_2(b: &mut Bencher) {
    let base: u32 = 200;
    let len = base * base;
    let mut expected: Vec<u32> = Vec::with_capacity(len as usize);
    for i in 0..len {
        expected.push((base - 1) - i % base);
    }
    b.iter(|| merge_sort_parallel(&expected, 2).unwrap())
}

fn bench_parallel_4(b: &mut Bencher) {
    let base: u32 = 200;
    let len = base * base;
    let mut expected: Vec<u32> = Vec::with_capacity(len as usize);
    for i in 0..len {
        expected.push((base - 1) - i % base);
    }
    b.iter(|| merge_sort_parallel(&expected, 4).unwrap())
}
*/
benchmark_group!(
    benches,
    bench_single_thread // ,
                        // bench_parallel_2,
                        // bench_parallel_4
);
benchmark_main!(benches);
