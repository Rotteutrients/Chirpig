use std::convert::From;
use std::error;
use std::fmt;
use std::path::PathBuf;

use argon2::password_hash::Error as PaswordHashError;

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
}

#[derive(Debug)]
pub enum RequestError {
    InvalidPassword,
}

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
