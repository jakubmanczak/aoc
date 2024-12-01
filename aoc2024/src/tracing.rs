use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn setup() {
    let s = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(s).unwrap();
}
