use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Fun};
use std::ops::{Range, RangeInclusive};

/// A test function that simply collapses the range by summing its elements.
fn calc(iter: impl Iterator<Item = u64>) -> u64 {
    iter.fold(0u64, |x, y| x.wrapping_add(y))
}

/// A range-like iterator that decides at initialization whether to go with an *inclusive* or
/// *non-inclusive* range under the hood depending on the upper bound value.
enum DynamicInclusiveRange<T> {
    Inclusive(RangeInclusive<T>),
    NonInclusive(Range<T>),
}

impl DynamicInclusiveRange<u64> {
    /// Initializes a dynamic range.
    pub fn new(from: u64, inclusive_to: u64) -> Self {
        if inclusive_to == u64::max_value() {
            DynamicInclusiveRange::Inclusive(from..=inclusive_to)
        } else {
            DynamicInclusiveRange::NonInclusive(from..(inclusive_to + 1))
        }
    }
}

impl Iterator for DynamicInclusiveRange<u64> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        match self {
            DynamicInclusiveRange::Inclusive(r) => r.next(),
            DynamicInclusiveRange::NonInclusive(r) => r.next(),
        }
    }
}

/// A helper function to prevent rust from optimizing out compile-time values.
#[inline(never)]
fn get_low_and_up(up: u64) -> impl FnMut() -> (u64, u64) {
    move || (black_box(1), black_box(up))
}

/// Creates a bencher that benches a non-inslucive range.
fn make_non_inclusive(up: u64) -> Fun<()> {
    Fun::new(&format!("non-inclusive {}", up), move |b, &()| {
        b.iter_batched(
            get_low_and_up(up),
            |(low, up)| calc(black_box(low..up)),
            BatchSize::SmallInput,
        );
    })
}

/// Creates a bencher that benches an inslucive range.
fn make_inclusive(up: u64) -> Fun<()> {
    Fun::new(&format!("inclusive {}", up), move |b, &()| {
        b.iter_batched(
            get_low_and_up(up),
            |(low, up)| calc(black_box(low..=up)),
            BatchSize::SmallInput,
        );
    })
}

/// Creates a bencher that benches DynamicInclusiveRange.
fn make_dynamic(up: u64) -> Fun<()> {
    Fun::new(&format!("dynamic {}", up), move |b, &()| {
        b.iter_batched(
            get_low_and_up(up),
            |(low, up)| calc(black_box(DynamicInclusiveRange::new(low, up))),
            BatchSize::SmallInput,
        );
    })
}

fn ranges(c: &mut Criterion) {
    c.bench_functions(
        "ranges",
        vec![
            make_non_inclusive(u64::max_value() - 1),
            make_inclusive(u64::max_value() - 1),
            make_dynamic(u64::max_value() - 1),
            make_non_inclusive(u64::max_value()),
            make_inclusive(u64::max_value()),
            make_dynamic(u64::max_value()),
        ],
        (),
    );
}

criterion_group!(benches, ranges);
criterion_main!(benches);
