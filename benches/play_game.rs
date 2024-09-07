use iai_test::play_game;

fn bench_play_game() {
    iai::black_box(for i in 1..=100 {
        play_game(i, false)
    });
}

iai::main!(bench_play_game);
