use std::sync::RwLock;

use tokio::sync::broadcast::{channel, Receiver, Sender};

pub struct UStream {}

pub struct UStreamConnect(pub Arc<RwLock<Vec<Option<UStream>>>>);
