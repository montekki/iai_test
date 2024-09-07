use iai_test::play_game;

fn bench_play_game() {
    for i in 1..=100 {
        play_game(i, false)
    }
    iai::black_box(());
}

iai::main!(bench_play_game);
