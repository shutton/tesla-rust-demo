use log::*;

fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter(Some("trd"), log::LevelFilter::Info)
        .filter(None, log::LevelFilter::Warn)
        .init();

    info!("Hello, world!");
}
