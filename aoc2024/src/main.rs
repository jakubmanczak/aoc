mod day01;
mod tracing;

fn main() {
    tracing::setup();

    day01::solve("input/01.txt");
}
