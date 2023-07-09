use async_once_cell::OnceCell;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Mutex};
use toml;

use crate::internal::crypt::{ChaCha20Poly1305, Crypt};
use crate::Result;

static SERVE_CONFIG: OnceCell<Mutex<Config>> = OnceCell::new();

macro_rules! try_init_config {
    // `()` indicates that the macro takes no argument.
    ($e:expr, $v:expr) => {
        if None == $e {
            $e = $v;
        }
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    symmetric_key: Option<String>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    conf: Option<String>,
}

impl Config {
    pub fn factory() -> Result<Self> {
        // include setting file
        let args = Args::parse();
        let path = PathBuf::from(&args.conf.unwrap_or("config.toml".to_string()));
        let config_str = std::fs::read_to_string(path.clone())
            .map_err(|_| crate::InternalError::FileIOError(path.clone()))?;
        let mut config: Config =
            toml::from_str(&config_str).map_err(|_| crate::InternalError::SerdeError(path))?;
        config.try_init()
    }

    fn try_init(mut self) -> Result<Self> {
        try_init_config!(
            self.symmetric_key,
            Some(ChaCha20Poly1305::generate().serialize())
        );

        Ok(self)
    }
}
