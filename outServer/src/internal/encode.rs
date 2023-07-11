use std::default;

use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};

use crate::{InternalError, Marker, Result};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Base64<M>(Marker<String, M>);
impl<M> Base64<M> {
    pub fn encode(data: &[u8]) -> Self {
        Self(Marker::<String, M>::new(
            general_purpose::STANDARD_NO_PAD.encode(data),
        ))
    }
    pub fn decode(&self) -> Result<Vec<u8>> {
        Ok(general_purpose::STANDARD_NO_PAD
            .decode(&self.0 .0)
            .map_err(|_| InternalError::Base64DecodeError)?)
    }

    pub fn to_string(self) -> String {
        self.0 .0
    }
}
