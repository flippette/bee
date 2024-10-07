#![feature(portable_simd)]

use core::hint::black_box;
use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use std::{
    collections::HashSet,
    fs,
    simd::{LaneCount, SupportedLaneCount},
};

fn bench_fits<'word, const N: usize>(
    bencher: &mut Bencher,
    dict: impl AsRef<[&'word [u8]]>,
    set: &[u8],
) where
    LaneCount<N>: SupportedLaneCount,
{
    bencher.iter(|| {
        for word in dict.as_ref() {
            bee::fits::<N>(word, set);
        }
    });
}

#[allow(clippy::unit_arg)]
fn bench(criterion: &mut Criterion) {
    let dict = fs::read("words.txt").expect("words.txt should exist");
    let dict = dict.split(u8::is_ascii_whitespace).collect::<Vec<_>>();
    let set = dict.iter().take(100).fold(HashSet::new(), |mut set, word| {
        word.iter().for_each(|ch| {
            set.insert(*ch);
        });
        set
    });
    let set = set.into_iter().collect::<Vec<_>>();

    criterion
        .benchmark_group("fits")
        .bench_with_input("fits1", &dict, |bencher, dict| {
            black_box(bench_fits::<1>(bencher, dict, &set))
        })
        .bench_with_input("fits2", &dict, |bencher, dict| {
            black_box(bench_fits::<2>(bencher, dict, &set))
        })
        .bench_with_input("fits4", &dict, |bencher, dict| {
            black_box(bench_fits::<4>(bencher, dict, &set))
        })
        .bench_with_input("fits8", &dict, |bencher, dict| {
            black_box(bench_fits::<8>(bencher, dict, &set))
        })
        .bench_with_input("fits16", &dict, |bencher, dict| {
            black_box(bench_fits::<16>(bencher, dict, &set));
        })
        .bench_with_input("fits32", &dict, |bencher, dict| {
            black_box(bench_fits::<32>(bencher, dict, &set));
        })
        .bench_with_input("fits64", &dict, |bencher, dict| {
            black_box(bench_fits::<64>(bencher, dict, &set));
        });
}

criterion_group!(benches, bench);
criterion_main!(benches);
