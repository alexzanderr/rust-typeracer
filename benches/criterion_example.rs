use criterion::{
    Criterion,
    BenchmarkId,
    black_box,
    criterion_group,
    criterion_main,
};

use soloud::*;
use typeracer::MusicPlayer;

pub const SKELER_TELATIV_SONG: &'static [u8] =
    include_bytes!("../static/audio/skeler-telaviv.mp3");
pub const PLAY_CS16_SOUND: &'static [u8] =
    include_bytes!("../static/audio/play_cs16.wav");
//
// pub fn bench_two_sum(criterion: &mut Criterion) {
//     criterion.bench_function("two+wum", |bencher| {
//         bencher.iter(move || {
//             two_sum(black_box(vec![10, 10, 10, 10, 10]), black_box(20))
//         });
//     });
// }
//
// pub fn bench_two_sum_2(criterion: &mut Criterion) {
//     let mut group = criterion.benchmark_group("two_sum");
//
//     let examples = for (target, nums) in [1, 5, 20].into_iter().zip(
//         [
//             vec![123i32, 123, 123, 123, 2, 8],
//             vec![10, 10, 10, 10, 10i32],
//             vec![]
//         ]
//             .iter()
//     ) {
//         group.bench_with_input(
//             BenchmarkId::from_parameter(target),
//             nums,
//             |bencher, nums| {
//                 // there's lots of different functions we could call on `bencher` to support
//                 // different use cases. `iter` is the simplest and works for most things.
//                 bencher.iter(|| {
//                     // anything inside this function body is going to get timed, so we really
//                     // don't want to add any correctness assertions here.
//                     // Also note the use of `black_box` (https://docs.rs/criterion/0.3.0/criterion/fn.black_box.html),
//                     // which ensures that this call doesn't get optimized away by the compiler. If your
//                     // benchmarks seem spooky fast, be suspicious.
//                     two_sum(black_box((&nums).to_vec()), black_box(target))
//                 });
//             }
//         );
//     };
// }
//
// criterion_group!(benches, bench_two_sum, bench_two_sum_2);
//
// criterion_main!(benches);
