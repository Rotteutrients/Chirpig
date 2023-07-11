use actix_rt::Runtime;
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    str::FromStr,
};

use crate::{Config, Result, ServerConfigError};

pub struct Server {
    socket: SocketAddr,
    tls: Option<(PathBuf, PathBuf)>,
}

impl Server {
    pub fn build() -> Result<Self> {
        Ok(Self {
            socket: SocketAddr::new(
                IpAddr::from_str(&Config::host()?).map_err(|_| ServerConfigError::InvalidHost)?,
                Config::port().map_err(|_| ServerConfigError::InvalidPort)?,
            ),
            tls: Config::tls()?,
        })
    }

    pub fn run(&self) {
        let rt = Runtime::new().unwrap();
        println!("Caller Block Outer");
        rt.block_on(async {
            println!("Caller Block Inner");
            let srv = HttpServer::new(|| App::new().service(index));
            let _status = {
                if let Some(cert) = &self.tls {
                    srv.bind(self.socket).unwrap().run().await
                } else {
                    srv.bind(self.socket).unwrap().run().await
                }
            };
        });
    }
}

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("welcome!!!")
}
