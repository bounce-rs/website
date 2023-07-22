#![deny(clippy::all)]
#![deny(missing_debug_implementations)]

use stellation_backend_cli::Cli;
use stellation_backend_tower::TowerEndpoint;

mod app;
use app::ServerApp;

#[cfg(stellation_embedded_frontend)]
#[derive(rust_embed::RustEmbed)]
#[folder = "$STELLATION_FRONTEND_BUILD_DIR"]
struct Frontend;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configures Logging
    stellation_backend_cli::trace::init_default("STELLATION_APP_SERVER_LOG");

    // Creates Endpoint
    let endpoint = TowerEndpoint::<ServerApp<_>>::new();

    #[cfg(stellation_embedded_frontend)]
    let endpoint =
        endpoint.with_frontend(stellation_backend_tower::Frontend::new_embedded::<Frontend>());

    // Starts Server
    Cli::builder().endpoint(endpoint).build().run().await?;

    Ok(())
}
