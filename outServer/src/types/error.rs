use std::convert::From;
use std::error;
use std::fmt;
use std::fmt::Display;
use std::path::PathBuf;
use std::sync::RwLock;

use argon2::password_hash::Error as PaswordHashError;

use crate::Config;

pub type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug)]
pub enum RuntimeError {
    ServerError(InternalError),
    RequestError(RequestError),
}

#[derive(Debug)]
pub enum InternalError {
    PasswordHash(PaswordHashError),
    PasswordAesCrypto(aes_gcm_siv::Error),
    FileIOError(PathBuf),
    SerdeError(PathBuf),
    Base64DecodeError,
    ServerConfigError(ServerConfigError),
    ServerError(String),
}

#[derive(Debug)]
pub enum RequestError {
    InvalidPassword,
}

#[derive(Debug)]
pub enum ServerConfigError {
    InvalidConfigFile,
    getConfigRuntime,
    InvalidHost,
    InvalidPort,
    InvalidCert,
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", 32u64)
    }
}
impl std::error::Error for RequestError {}

impl From<InternalError> for RuntimeError {
    fn from(item: InternalError) -> Self {
        RuntimeError::ServerError(item)
    }
}

impl From<RequestError> for RuntimeError {
    fn from(item: RequestError) -> Self {
        RuntimeError::RequestError(item)
    }
}

impl From<PaswordHashError> for RuntimeError {
    fn from(item: PaswordHashError) -> Self {
        if item == PaswordHashError::Password {
            RuntimeError::RequestError(RequestError::InvalidPassword)
        } else {
            RuntimeError::ServerError(InternalError::PasswordHash(item))
        }
    }
}

impl From<aes_gcm_siv::Error> for RuntimeError {
    fn from(item: aes_gcm_siv::Error) -> Self {
        RuntimeError::ServerError(InternalError::PasswordAesCrypto(item))
    }
}

impl From<RwLock<Config>> for RuntimeError {
    fn from(item: RwLock<Config>) -> Self {
        RuntimeError::ServerError(InternalError::ServerError(
            format!("{:?}", item).to_string(),
        ))
    }
}

impl From<ServerConfigError> for RuntimeError {
    fn from(item: ServerConfigError) -> Self {
        RuntimeError::ServerError(InternalError::ServerConfigError(item))
    }
}
