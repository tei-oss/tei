use color_eyre::eyre::Result;
use tracing::{subscriber::set_global_default, Level};

pub fn install() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv().expect(".env file not found");

    let collector = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();

    set_global_default(collector)?;

    Ok(())
}
