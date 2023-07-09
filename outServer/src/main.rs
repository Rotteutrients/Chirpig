use std::env;

mod entry {
    pub mod chirp;
    pub mod member;
}

mod types {
    pub mod config;
    pub mod error;
}
mod internal {
    pub mod crypt;
}
pub use entry::chirp::Chirp;
pub use types::error::{InternalError, Result};

use crate::internal::crypt::Crypt;
use crate::types::config::Config;

fn main() {
    let config = Config::factory();
    let mut body = b"message testing".to_vec();
    let enc = internal::crypt::ChaCha20Poly1305::generate();
    if let Ok(enced) = enc.encrypt(&body) {
        let data = enc.decrypt(&enced);
        println!("Decrypt data: {:?}", data);
    }

    //println!("{:?}", Chirp::chirp());
    let mc =
        entry::member::MemberCredentials::new("rotteutrients@xtrap.app", "some_good_pass").unwrap();
    println!("{:#?} {:?}", mc, mc.verify("some_good_pass"));
}
