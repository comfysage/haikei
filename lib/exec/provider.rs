use crate::prelude::*;
use crate::config::Config;

enum Provider {
    SWWW,
    HSETROOT,
    FEH,
    Unknown(String),
}

impl Provider {
    pub fn from(name: &str) -> Self {
        match name {
            "swww" => Self::SWWW,
            custom => Self::Unknown(custom.to_string()),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Self::SWWW => "swww img -t outer --transition-step 250 --transition-fps 60",
            Self::HSETROOT => "hsetroot -cover",
            Self::FEH => "feh --bg-fill",
            Self::Unknown(custom) => custom,
        }.to_string()
    }
}

pub fn get_provider_cmd() -> Result<String> {
    let cfg = Config::new()?;
    let provider = Provider::from(&cfg.main.provider);
    Ok(provider.to_string())
}
