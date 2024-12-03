mod day01;
mod day02;
mod day03;
mod tracing;

fn main() {
    tracing::setup();

    day01::solve();
    day02::solve();
    day03::solve();
}
