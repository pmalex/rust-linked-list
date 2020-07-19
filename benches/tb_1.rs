use criterion::{black_box, criterion_group, criterion_main, Criterion};

use linked_list::one_linked_list::List;

pub fn mulpitple_push(c: &mut Criterion) {
    let mut list = List::<u32>::new();

    c.bench_function("Multiple push_front", |b| b.iter(|| {
        for i in 1..100 {
            list.push_front(i);
        }
    }));
}

criterion_group!(benches, mulpitple_push);
criterion_main!(benches);