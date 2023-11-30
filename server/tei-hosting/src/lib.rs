use color_eyre::eyre::Result;
use tracing::subscriber::set_global_default;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter};

pub fn install() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv().expect(".env file not found");

    let collector = tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env());

    set_global_default(collector)?;

    Ok(())
}
