use sha2::Digest;

pub trait TypeSignature {
    fn signature(&self) -> Vec<u8>;
    fn as_hashed(raw_sig: Vec<u8>) -> Vec<u8> {
        let mut hasher = sha2::Sha256::new();
        hasher.update(raw_sig);
        hasher.finalize().to_vec()
    }
}
