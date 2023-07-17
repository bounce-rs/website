mod app;
use app::App;
use tracing::metadata::LevelFilter;

fn main() {
    // Configures Logging
    stellation_frontend::trace::init_default(LevelFilter::INFO);

    // Starts Application
    stellation_frontend::Renderer::<App>::new().render();
}
