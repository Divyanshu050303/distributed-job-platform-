use sha2::{Digest, Sha256};

pub struct TokenHashService;

impl TokenHashService {
    pub fn hash(token: &str) -> String {
        let mut hasher = Sha256::new();

        hasher.update(token);

        format!("{:x}", hasher.finalize())
    }
}
