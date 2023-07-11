#[derive(Debug)]
pub struct Followee {
    resouse: Url,
    public_key: String,
    credential: FolloweeCredentials,
}

#[derive(Debug)]
pub struct FolloweeCredentials {
    private_key: String,
    symmetric_key: Option<String>,
    expires: Option<String>,
    synmmetric_key_next: Option<String>,
    expires_next: Option<String>,
}
