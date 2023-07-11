use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    ChaCha20Poly1305 as ChaCha20Poly1305Chiper,
};
use crypto::elliptic_curve::{PublicKey, SecretKey};
use generic_array::GenericArray;
use k256::Secp256k1;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};

use crate::internal::encode::Base64;
use crate::{InternalError, Marker, Result};

pub trait Crypt {
    const BLOCK_CHANK_SIZE: usize = 16;

    fn seed() -> ChaCha20Rng {
        ChaCha20Rng::from_entropy()
    }
    fn generate() -> Self;
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChaCha20Poly1305(Vec<u8>);
pub type ChaCha20Poly1305Crypted<T> = Marker<T, ChaCha20Poly1305>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Secp256k1Secret(Vec<u8>);

impl Crypt for ChaCha20Poly1305 {
    fn generate() -> Self {
        Self(
            ChaCha20Poly1305Chiper::generate_key(&mut Self::seed())
                .as_slice()
                .into(),
        )
    }
}

impl Crypt for Secp256k1Secret {
    fn generate() -> Self {
        Self(
            SecretKey::<Secp256k1>::random(&mut Self::seed())
                .to_bytes()
                .as_slice()
                .iter()
                .map(|&t| t)
                .collect(),
        )
    }
}

impl ChaCha20Poly1305 {
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let key = GenericArray::from_slice(&self.0);

        let cipher = ChaCha20Poly1305Chiper::new(&key);
        let nonce = ChaCha20Poly1305Chiper::generate_nonce(&mut Self::seed());
        let ciphertext = cipher.encrypt(&nonce, data)?;
        let mut result = nonce.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let key = GenericArray::from_slice(&self.0);

        let cipher = ChaCha20Poly1305Chiper::new(&key);
        Ok(cipher.decrypt(GenericArray::from_slice(&data[..12]), &data[12..])?)
    }

    pub fn serialize(&self) -> Result<Base64<Self>> {
        Ok(Base64::encode(&self.0))
    }

    pub fn deserialize(str: &Base64<Self>) -> Result<Self> {
        Ok(Self(Base64::decode(str)?))
    }
}

impl Secp256k1Secret {
    pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        todo!();
    }
    pub fn verify(&self, data: &[u8]) -> Result<Vec<u8>> {
        todo!();
    }
    pub fn serialize(
        &self,
        crypt: &ChaCha20Poly1305,
    ) -> Result<Base64<ChaCha20Poly1305Crypted<Self>>> {
        Ok(Base64::encode(&crypt.encrypt(&self.0)?))
    }
    pub fn deserialize(
        str: &Base64<ChaCha20Poly1305Crypted<Self>>,
        crypt: &ChaCha20Poly1305,
    ) -> Result<Self> {
        Ok(Self(crypt.decrypt(&Base64::decode(&str)?)?))
    }
}
