#![feature(test)]

extern crate test;

use rand;
use rand::{thread_rng, Rng};
use specsort::SpecSort;
use test::Bencher;

fn sort_bool(s: &mut [bool]) {
    if cfg!(feature="bench_std") {
        s.sort_unstable()
    } else {
        <bool as SpecSort>::sort_unstable(s);
    }
}

fn bench_random(b: &mut Bencher, size: usize) {
    unsafe {
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(rand::random());
        }

        b.iter(|| {
            let mut v = v.clone();
            sort_bool(&mut v);
            *v.get_unchecked(0)
        })
    }
}

fn bench_sorted(b: &mut Bencher, size: usize) {
    unsafe {
        let mut v: Vec<bool> = Vec::with_capacity(size);
        let mid = thread_rng().gen_range(0, size);
        let mut v = Vec::with_capacity(size);
        for _ in 0..mid {
            v.push(false);
        }
        for _ in mid..size {
            v.push(true);
        }

        b.iter(|| {
            let mut v = v.clone();
            sort_bool(&mut v);
            *v.get_unchecked(0)
        })
    }
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
