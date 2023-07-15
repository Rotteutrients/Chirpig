mod entry {
    pub mod chirp;
    pub mod member;
}

mod types {
    pub mod config;
    pub mod error;
    pub mod marker;
}
mod internal {
    pub mod crypt;
    pub mod encode;
    pub mod hash;
}

mod external {
    pub mod export;
    pub mod server;
}

pub use entry::chirp::Chirp;
pub use types::error::{InternalError, Result, ServerConfigError};
pub use types::marker::Marker;

use crate::external::server::Server;
use crate::internal::crypt::Crypt;
use crate::types::config::Config;

fn main() {
    if let Ok(_) = Config::factory() {
        println!("Starting server");
        let status: Result<()> = (|| {
            println!("Starting server...");
            Server::build()?.run();
            Ok(())
        })();
    } else {
        println!("Failed to start server");
    }

    /*
        let body = b"message testing".to_vec();
        let enc = internal::crypt::ChaCha20Poly1305::generate();
        if let Ok(enced) = enc.encrypt(&body) {
            let data = enc.decrypt(&enced);
            println!("Decrypt data: {:?}", data);
        }

        //println!("{:?}", Chirp::chirp());
        let mc =
            entry::member::MemberCredentials::new("rotteutrients@example.com", "example password").unwrap();
        println!("{:#?} {:?}", mc, mc.verify("unmatch password"));
    */
}
