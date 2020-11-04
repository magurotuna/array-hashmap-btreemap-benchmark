use array_hashmap_btreemap_benchmark::{
    array_contains, array_contains_binary_search, btreemap_contains, hashmap_contains,
};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_search_element(c: &mut Criterion) {
    let mut group = c.benchmark_group("Search an element");
    for key in [
        "AbortController",
        "MessageEvent",
        "WritableStream",
        "UNEXISTENT_ELEMENT",
    ]
    .iter()
    {
        group.bench_with_input(BenchmarkId::new("Array", key), key, |b, key| {
            b.iter(|| array_contains(key))
        });
        group.bench_with_input(BenchmarkId::new("ArrayBinarySearch", key), key, |b, key| {
            b.iter(|| array_contains_binary_search(key))
        });
        group.bench_with_input(BenchmarkId::new("BTreeMap", key), key, |b, key| {
            b.iter(|| btreemap_contains(key))
        });
        group.bench_with_input(BenchmarkId::new("HashMap", key), key, |b, key| {
            b.iter(|| hashmap_contains(key))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_search_element);
criterion_main!(benches);
