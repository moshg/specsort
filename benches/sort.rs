#![feature(test)]

extern crate test;

use rand;
use specsort::SpecSort;
use test::Bencher;

#[cfg(feature = "bench_bool")]
fn sort(s: &mut [bool]) {
    if cfg!(feature = "bench_std") {
        s.sort_unstable()
    } else if cfg!(feature = "bench_by_key") {
        <bool as SpecSort>::sort_by_me(s, |b| *b)
    } else {
        <bool as SpecSort>::sort_unstable(s);
    }
}

#[cfg(feature = "bench_u8")]
fn sort(s: &mut [u8]) {
    if cfg!(feature = "bench_std") {
        s.sort_unstable()
    } else if cfg!(feature = "bench_by_key") {
        <u8 as SpecSort>::sort_by_me(s, |n| *n)
    } else {
        <u8 as SpecSort>::sort_unstable(s);
    }
}

fn bench_random(b: &mut Bencher, size: usize) {
    let mut v = Vec::with_capacity(size);
    for _ in 0..size {
        v.push(rand::random());
    }

    b.iter(|| {
        let mut v = v.clone();
        sort(&mut v);
        v
    })
}

fn bench_sorted(b: &mut Bencher, size: usize) {
    let mut v = Vec::with_capacity(size);
    for _ in 0..size {
        v.push(rand::random());
    }
    v.sort();

    b.iter(|| {
        let mut v = v.clone();
        sort(&mut v);
        v
    })
}

#[bench]
fn bench_random_100(b: &mut Bencher) {
    bench_random(b, 100)
}

#[bench]
fn bench_random_10000(b: &mut Bencher) {
    bench_random(b, 10_000)
}

#[bench]
fn bench_random_1000000(b: &mut Bencher) {
    bench_random(b, 1_000_000)
}

#[bench]
fn bench_sorted_100(b: &mut Bencher) {
    bench_sorted(b, 100)
}

#[bench]
fn bench_sorted_10000(b: &mut Bencher) {
    bench_sorted(b, 10_000)
}

#[bench]
fn bench_sorted_1000000(b: &mut Bencher) {
    bench_sorted(b, 1_000_000)
}
