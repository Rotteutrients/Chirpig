use actix_web::web::Bytes;
use chrono::Utc;
use chrono::{DateTime, Duration};
use futures_core::stream::Stream;
use futures_core::task::Context;
use futures_core::task::Poll;
use futures_core::task::Waker;
use rand_chacha::rand_core::le;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::{Mutex, RwLock};

use crate::types::error::RequestError;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Event {}

#[derive(Debug, Clone)]
pub struct EventSender(pub Arc<RwLock<Vec<Option<EventSenderInner>>>>);

#[derive(Debug, Clone)]
pub struct EventSenderInner {
    tx: Sender<String>,
    waker: Arc<Mutex<Option<Waker>>>,
}

pub struct EventReceiver {
    rx: Receiver<String>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl EventSender {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(vec![None])))
    }
    pub async fn connect(&self) -> EventReceiver {
        let (tx, rx) = mpsc::channel(10);

        let waker = Arc::new(Mutex::new(None));
        {
            let mut this = self.0.write().await;

            (|item| match this.iter_mut().find(|x| x.is_none()) {
                Some(none) => *none = Some(item),
                None => this.push(Some(item)),
            })(EventSenderInner {
                tx: tx,
                waker: waker.clone(),
            });
        }
        EventReceiver {
            rx: rx,
            waker: waker,
        }
    }

    pub async fn wake(&self) {
        let wakes = self.0.read().await;
        let txs: Vec<_> = wakes
            .iter()
            .filter_map(|i| {
                if let Some(j) = i {
                    println!("{:?}\n{:?}", j.waker, j);
                    Some(j.tx.send(Utc::now().format("data: %Y-%m-%d %H:%M:%S\n\n").to_string()))
                } else {
                    None
                }
            })
            .collect();

        println!("Length: {}", &txs.len());
        futures::future::join_all(txs).await;
    }
}

impl Stream for EventReceiver {
    type Item = Result<Bytes, RequestError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        println!("Called");
        match self.rx.poll_recv(cx) {
            Poll::Pending => {
                println!("Pending");
                Poll::Pending
            }
            Poll::Ready(None) => {
                println!("Ready(None)");
                Poll::Ready(None)
            }
            Poll::Ready(Some(m)) => {
                println!("Ready(Some({}))", &m);
                Poll::Ready(Some(Ok(m.into())))
            }
        }
    }
}
