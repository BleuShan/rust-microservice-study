use crate::prelude::*;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Application {
    port: u8,
}

impl Application {
    pub fn init() -> Result<Self> {
        color_eyre::install()?;
        dotenv::dotenv().ok();
        let app = Application::parse();
        Ok(app)
    }
}
