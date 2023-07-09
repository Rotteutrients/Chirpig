use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Follower {
    resouse: Url,
    credential: FollowerCredentials,
}

#[derive(Debug)]
pub struct FollowerCredentials {
    public_key: String,
    last_required_datetime: DateTime<Utc>,
}
