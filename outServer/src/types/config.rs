use actix_web::body::MessageBody;
use clap::Parser;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::default;
use std::fs::OpenOptions;
use std::io::Write;
use std::ops::Deref;
use std::{path::PathBuf, sync::Mutex, sync::RwLock};
use toml;

use crate::internal::crypt::{ChaCha20Poly1305, Crypt};
use crate::internal::encode::Base64;

use crate::{InternalError, Result, ServerConfigError};

static SERVE_CONFIG: OnceCell<RwLock<Config>> = OnceCell::new();
static SERVE_CONFIG_PATH: OnceCell<RwLock<PathBuf>> = OnceCell::new();

macro_rules! try_init_config {
    // `()` indicates that the macro takes no argument.
    ($e:expr, $v:expr,  $change:expr) => {
        if None == $e && None != $v {
            $e = $v;
            $change |= true;
        }
    };
}

macro_rules! get_config {
    ($elm:ident) => {
        SERVE_CONFIG
            .get()
            .ok_or(ServerConfigError::InvalidConfigFile)?
            .read()
            .map_err(|_| ServerConfigError::InvalidConfigFile)?
            .deref()
            .$elm
            .as_ref()
    };
}

macro_rules! get_config_function {
    ($func:ident, $typ:ty, $elm:ident) => {
        pub fn $func() -> Result<$typ> {
            Ok(get_config!($elm)
                .ok_or(ServerConfigError::InvalidConfigFile)?
                .clone())
        }
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    symmetric_key: Option<Base64<ChaCha20Poly1305>>,
    host: Option<String>,
    port: Option<u16>,
    tls: Option<Vec<(PathBuf, PathBuf)>>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    conf: Option<String>,
    #[arg(long)]
    init: bool,
}

impl Config {
    pub fn factory() -> Result<()> {
        // include execute args
        let args = Args::parse();

        // include setting file
        let path = PathBuf::from(&args.conf.unwrap_or("config.toml".to_string()));
        let config_dir: PathBuf = path
            .parent()
            .ok_or(crate::InternalError::FileIOError(path.clone()))?
            .into();
        SERVE_CONFIG_PATH
            .set(RwLock::new(config_dir.clone()))
            .map_err(|_| crate::InternalError::FileIOError(config_dir.clone()))?;

        let config_str = match std::fs::read_to_string(path.clone()) {
            Ok(s) => {
                if args.init != true {
                    Ok(s)
                } else {
                    Err(crate::InternalError::FileIOError(path.clone()))
                }
            }
            Err(_) => {
                if args.init == true {
                    Ok("".into())
                } else {
                    Err(crate::InternalError::FileIOError(path.clone()))
                }
            }
        }?;
        let mut config: Config = toml::from_str(&config_str)
            .map_err(|_| crate::InternalError::SerdeError(path.clone()))?;

        let initialized = config.try_init()?;
        if initialized {
            std::fs::write(
                &path,
                toml::to_string(&config)
                    .map_err(|_| crate::InternalError::SerdeError(path.clone()))?,
            )
            .map_err(|_| crate::InternalError::FileIOError(path.clone()))?;

            //create .gitignore (looks evil)
            let mut require_write = true;
            let ignore_path = config_dir.join(".gitignore");
            let file_name = path
                .file_name()
                .ok_or(crate::InternalError::FileIOError(path.clone()))?
                .to_str()
                .ok_or(crate::InternalError::FileIOError(path.clone()))?
                .to_string();
            if let Ok(ignore) = std::fs::read_to_string(ignore_path.clone()) {
                for line in ignore.lines() {
                    if !line.contains("#") && line.contains(&file_name) {
                        require_write &= false;
                    }
                }
            }
            if require_write {
                let _ = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&ignore_path)
                    .map_err(|_| crate::InternalError::FileIOError(ignore_path.clone()))?
                    .write_all(&file_name.try_into_bytes().unwrap());
            }
        }

        Ok(())
    }

    fn try_init(&mut self) -> Result<bool> {
        let mut change = false;
        try_init_config!(self.host, Some("127.0.0.1".to_string()), change);
        try_init_config!(self.port, Some(8080u16), change);
        try_init_config!(self.tls, Some(Vec::<(PathBuf, PathBuf)>::default()), change);
        try_init_config!(
            self.symmetric_key,
            Some(ChaCha20Poly1305::generate().serialize()?),
            change
        );
        SERVE_CONFIG.set(RwLock::new(self.clone()))?;
        Ok(change)
    }
}

impl Config {
    get_config_function!(host, String, host);
    get_config_function!(port, u16, port);
    pub fn tls() -> Result<Option<(PathBuf, PathBuf)>> {
        let mut t = get_config!(tls)
            .ok_or(ServerConfigError::InvalidConfigFile)?
            .clone();
        match t.len() {
            0 => Ok(None),
            1 => Ok({
                let tp = t.pop().unwrap();
                Some((
                    SERVE_CONFIG_PATH
                        .get()
                        .ok_or(ServerConfigError::InvalidConfigFile)?
                        .read()
                        .map_err(|_| ServerConfigError::InvalidConfigFile)?
                        .deref()
                        .join(tp.0),
                    SERVE_CONFIG_PATH
                        .get()
                        .ok_or(ServerConfigError::InvalidConfigFile)?
                        .read()
                        .map_err(|_| ServerConfigError::InvalidConfigFile)?
                        .deref()
                        .join(tp.1),
                ))
            }),
            _ => Err(ServerConfigError::InvalidCert.into()),
        }
    }
    pub fn get_symmetric_key() -> Result<ChaCha20Poly1305> {
        ChaCha20Poly1305::deserialize(
            get_config!(symmetric_key).ok_or(ServerConfigError::InvalidConfigFile)?,
        )
    }
}
