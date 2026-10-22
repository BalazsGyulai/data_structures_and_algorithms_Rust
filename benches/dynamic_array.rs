
// BenchmarkId::new(name, n): the name becomes a line in the graph and n the x-axis.
// black_box: stops the compiler from optimizing work away when it can see the result is never used. Without it you can measure 0 ns.
// iter_batched(setup, routine, …) (used for the pop benchmarks): setup builds a full array, and only routine is timed.

// Run it:
// > cargo bench --bench dynamic_array

// For a quick first try:
// > cargo bench --bench dynamic_array -- --warm-up-time 0.5 --measurement-time 2 --sample-size 20

// Run only some benchmarks:
// > cargo bench --bench dynamic_array -- "remove"

// Open the report with all the graphs:
// > open target/criterion/report/index.html


use std::hint::black_box;

use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use exercises::data_structure::dynamic_array::DynamicArray;

const SIZES: [usize; 5] = [1_000, 2_000, 4_000, 8_000, 16_000];

fn filled(n: usize) -> DynamicArray {
    let mut arr = DynamicArray::new();
    for i in 0..n {
        arr.push_back(i as isize);
    }
    return arr;
}

fn fill(c: &mut Criterion) {
    let mut group = c.benchmark_group("fill n elements"); // ← this is `group`

    for n in SIZES {
        group.throughput(Throughput::Elements(n as u64));

        group.bench_with_input(BenchmarkId::new("push_back", n), &n, |b, &n| {
            b.iter(|| {
                let mut arr = DynamicArray::new();

                for i in 0..n {
                    arr.push_back(black_box(i as isize));
                }

                arr
            });
        });

        group.bench_with_input(BenchmarkId::new("push_front", n), &n, |b, &n| {
            b.iter(|| {
                let mut arr = DynamicArray::new();

                for i in 0..n {
                    arr.push_front(black_box(i as isize));
                }

                arr
            });
        });

        group.bench_with_input(BenchmarkId::new("insert_middle", n), &n, |b, &n| {
            b.iter(|| {
                let mut arr = DynamicArray::new();

                for i in 0..n {
                    arr.insert(arr.size() / 2, black_box(i as isize));
                }

                arr
            });
        });

    }

    group.finish();
}

fn drain (c: &mut Criterion) {
    let mut group = c.benchmark_group("remove all n elements");

    for n in SIZES {
        group.throughput(Throughput::Elements(n as u64));

        group.bench_with_input(BenchmarkId::new("pop_back", n), &n, |b, &n| {
            b.iter_batched(|| filled(n),
            |mut arr| {
                while let Some(x) = arr.pop_back() {
                    black_box(x);
                }

                arr
            }
            , BatchSize::LargeInput);
        });
        group.bench_with_input(BenchmarkId::new("pop_front", n), &n, |b, &n| {
            b.iter_batched(|| filled(n),
            |mut arr| {
                while let Some(x) = arr.pop_front() {
                    black_box(x);
                }

                arr
            }
            , BatchSize::LargeInput);
        });
    }
}

fn lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("one lookup in n elements");

    for n in SIZES {
        let arr = filled(n);

        group.bench_with_input(BenchmarkId::new("get_middle", n), &arr, |b, arr| {
            b.iter(|| arr.get(black_box(n / 2)))
        });

        group.bench_with_input(BenchmarkId::new("contains_missing", n), &arr, |b, arr| {
            b.iter(|| arr.contains(black_box(-1)))
        });
    }

    group.finish();
}

criterion_group!(benches, fill, drain, lookup); // list your benchmark functions here
criterion_main!(benches); // this generates `main` for you
