use chrono::{DateTime, Utc};
use url::Url;

use argon2::Argon2;
use crypto::elliptic_curve::{PublicKey, SecretKey};
use crypto::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use k256::pkcs8::EncodePrivateKey;
use k256::Secp256k1 as Secp256k1Chiper;
use rand::SeedableRng;
use rand_chacha::ChaCha12Rng;

use crate::internal::crypt::{Crypt, Secp256k1Secret};
use crate::{InternalError, Result};

#[derive(Debug)]
pub struct Member {
    resouse: Url,
    public_key: String,
    timestamp: DateTime<Utc>,
    profile: MemberProfile,
    credential: MemberCredentials,
}

#[derive(Debug)]
pub struct MemberProfile {
    name: String,
    bio: String,
    location: String,
    url: String,
}

#[derive(Debug)]
pub struct MemberCredentials {
    email: String,
    password: String,
    private_key: String,
}

impl MemberCredentials {
    pub fn new(email: &str, password: &str) -> Result<Self> {
        Ok(Self {
            email: email.to_string(),
            password: Self::password_hash(password)?,
            private_key: Self::private_key()?,
        })
    }

    pub fn verify(&self, password: &str) -> Result<()> {
        let expect = PasswordHash::new(&self.password)?;
        Argon2::default().verify_password(password.as_bytes(), &expect)?;
        Ok(())
    }

    fn password_hash(password: &str) -> Result<String> {
        let salt = SaltString::generate(ChaCha12Rng::from_entropy());
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(password_hash)
    }

    fn private_key() -> Result<String> {
        let _private_key = Secp256k1Secret::generate();
        //println!("{:?}", private_key.to_pkcs8_pem(base64ct::LineEnding::LF));
        Ok(String::default())
    }
}
