use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub fn init() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stdout))
        .with(
            EnvFilter::from_default_env()
                .add_directive("info".parse().unwrap())
                .add_directive("rustobot=debug".parse().unwrap()),
        )
        .init();
}
