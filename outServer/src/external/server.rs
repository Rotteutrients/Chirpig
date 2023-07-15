use std::{
    fs::File,
    io::BufReader,
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    str::FromStr,
    time::Duration,
};

use crate::external::export::{Event, EventReceiver, EventSender};
use crate::{Config, Result, ServerConfigError};
use actix_rt::Runtime;
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
//use std::sync::mpsc::channel;
use tokio::sync::{mpsc, RwLock};

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
            let event = EventSender::new();

            let data = actix_web::web::Data::new(event);
            let data2 = data.clone();
            tokio::task::spawn(async move {
                // タスクを記述する
                loop {
                    tokio::time::sleep(Duration::from_secs(5u64)).await;
                    data2.wake().await;
                }
            });
            println!("Caller Block Inner");
            let srv = HttpServer::new(move || {
                App::new()
                    .app_data(data.clone())
                    .service(index)
                    .service(socket)
                    .service(stream)
            });
            let _status = {
                if let Some(cert) = &self.tls {
                    srv.bind_rustls(self.socket, self.build_tls(cert))
                        .unwrap()
                        .run()
                        .await
                } else {
                    srv.bind(self.socket).unwrap().run().await
                }
            };
        });
    }

    fn build_tls(&self, cert: &(PathBuf, PathBuf)) -> ServerConfig {
        let config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth();

        // load TLS key/cert files
        let cert_file = &mut BufReader::new(File::open(&cert.0).unwrap());
        let key_file = &mut BufReader::new(File::open(&cert.1).unwrap());

        // convert files to key/cert objects
        let cert_chain = certs(cert_file)
            .unwrap()
            .into_iter()
            .map(Certificate)
            .collect();
        let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
            .unwrap()
            .into_iter()
            .map(PrivateKey)
            .collect();

        // exit if no keys could be parsed
        if keys.is_empty() {
            eprintln!("Could not locate PKCS 8 private keys.");
            std::process::exit(1);
        }

        config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
    }
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("welcome!!!"))
}

#[get("/socket")]
async fn socket() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("welcome!!!"))
}

#[get("/stream")]
async fn stream(sender: actix_web::web::Data<EventSender>) -> impl Responder {
    let receiver = sender.connect().await;
    HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(receiver)
}
