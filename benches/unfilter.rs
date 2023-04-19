use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use png::{filter::unfilter, BytesPerPixel, FilterType};
use rand::{thread_rng, Rng};

fn bench_unfilter(c: &mut Criterion) {
    let filter_types = [
        FilterType::Sub,
        FilterType::Up,
        FilterType::Avg,
        FilterType::Paeth,
    ];
    let bpps = [
        BytesPerPixel::One,
        BytesPerPixel::Two,
        BytesPerPixel::Three,
        BytesPerPixel::Four,
        BytesPerPixel::Six,
        BytesPerPixel::Eight,
    ];

    let mut group = c.benchmark_group("unfilter");
    group.noise_threshold(0.05).sample_size(500);
    for filter in filter_types {
        for tbpp in bpps {
            group.throughput(Throughput::Bytes(1000 * tbpp.into_usize() as u64));
            group.bench_with_input(
                format!("{:?}-{}bpp", filter, tbpp.into_usize()),
                &(filter, tbpp),
                |b, (filter, tbpp)| {
                    let mut rng = thread_rng();
                    let mut previous = vec![0; 1000 * tbpp.into_usize()];
                    let mut current = vec![0; 1000 * tbpp.into_usize()];
                    rng.fill(&mut previous[..]);
                    rng.fill(&mut current[..]);
                    b.iter(move || {
                        unfilter(*filter, *tbpp, &previous, &mut current);
                    })
                },
            );
        }
    }
}

criterion_group!(benches, bench_unfilter);
criterion_main!(benches);
