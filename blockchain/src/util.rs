use sha256::digest;
use uuid::Uuid;
use hex::{
    FromHexError,
    decode
};
use ed25519_compact::{
    Error,
    PublicKey,
    Signature
};

pub struct Util;

pub enum VerifySigErr {
    DecodeStrError(FromHexError),
    DecodeHexError(Error)
}

impl From<FromHexError> for VerifySigErr {
    fn from(value: FromHexError) -> Self {
        VerifySigErr::DecodeStrError(value)
    }
}

impl From<Error> for VerifySigErr {
    fn from(value: Error) -> Self {
        VerifySigErr::DecodeHexError(value)
    }
}

impl Util {
    pub fn id() -> Uuid {
        Uuid::new_v4()
    }

    pub fn verify_signature(
        from_public_key: &String,
        message: &String,
        from_signature: &String
    ) -> Result<bool, VerifySigErr> {

        let public_key = decode(from_public_key)?;
        let ed25519_public_key = PublicKey::from_slice(&public_key)?;

        let signature = decode(from_signature)?;
        let ed25519_signature = &Signature::from_slice(&signature)?;

        Ok(ed25519_public_key
            .verify(message, ed25519_signature)
            .is_ok())
    }

    pub fn hash(data: &String) -> String {
        digest(data.as_bytes())
    }
}