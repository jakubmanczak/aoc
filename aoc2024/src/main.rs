mod day01;
mod day02;
mod tracing;

fn main() {
    tracing::setup();

    day01::solve();
    day02::solve();
}
