use frontend::listener::Listener;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub mod auth;
pub mod backend;
pub mod channel;
pub mod frontend;
pub mod net;
pub mod state;

// pub mod plugin;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let mut listener = Listener::new("0.0.0.0:6432");
    listener.listen().await.unwrap();
}
