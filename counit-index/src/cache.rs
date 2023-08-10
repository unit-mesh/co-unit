use uuid::Uuid;

/// Generate a content hash from the embedding data, and pin it to
/// the containing file's content id.
pub fn cache_key(data: &str) -> String {
    let id = {
        let mut bytes = [0; 16];
        let mut hasher = blake3::Hasher::new();
        hasher.update(data.as_ref());
        bytes.copy_from_slice(&hasher.finalize().as_bytes()[16..32]);
        Uuid::from_bytes(bytes).to_string()
    };
    id
}
