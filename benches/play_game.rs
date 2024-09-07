use iai_callgrind::{
    library_benchmark, library_benchmark_group, main, FlamegraphConfig, LibraryBenchmarkConfig,
};
use iai_test::play_game;
use std::hint::black_box;

#[library_benchmark]
fn bench_play_game() {
    for i in 1..=100 {
        play_game(i, false)
    }
    black_box(());
}

library_benchmark_group!(name = play_game_group; benchmarks = bench_play_game);

main!(
    config = LibraryBenchmarkConfig::default().flamegraph(FlamegraphConfig::default());
    library_benchmark_groups = play_game_group
);
