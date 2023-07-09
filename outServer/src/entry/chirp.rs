use chrono::{Utc, DateTime};
use url::Url;

const MESSAGE_CLIP_CNT: usize = 4;

#[derive(Debug)]
pub struct Chirp {
    id: u64,
    user: Url,
    content: ChirpContent,
    timestamp: DateTime<Utc>,
    signature: String
}

#[derive(Debug)]
enum ChirpContent {
    Obj(ChirpMessage),
    Re(Url, Box<Chirp>),
    Qt(Url, Box<Chirp>, ChirpMessage),
}

#[derive(Debug)]
pub struct ChirpMessage {
    message: String,
    clip: [Option<Url>; MESSAGE_CLIP_CNT]
}

impl Chirp {
    pub fn chirp() -> Self {
        let user = Url::parse("https://rotteutrients.xtrap.app/rotteutrients").unwrap();
        let time = Utc::now();
        let body = "".to_string();
        let signature = "".to_string();
        let id = 1; //insertQuery
        let clip = [];

        Self {
            id: id,
            user: user,
            content: ChirpContent::Obj(ChirpMessage::new(body, &clip)),
            timestamp: time,
            signature: signature
        }
    }
}

impl ChirpMessage {
    pub fn new(body: String, _clip: &[Url]) -> Self {
        Self {
            message: body,
            clip: [None,None,None,None]
        }
    }
}