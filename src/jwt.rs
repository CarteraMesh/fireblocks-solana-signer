use {
    // reqwest::http::Extensions,
    jsonwebtoken::{Algorithm, EncodingKey, Header, errors as jwterrors},
    rand::Rng,
    serde::{Deserialize, Serialize},
    sha2::{Digest, Sha256},
    std::{
        fmt::Debug,
        time::{SystemTime, UNIX_EPOCH},
    },
    thiserror::Error,
};

const EXPIRY: u64 = 55;

#[derive(Clone)]
pub struct JwtSigner {
    key: EncodingKey,
    api_key: String,
}

impl JwtSigner {
    pub fn new(key: EncodingKey, api_key: &str) -> Self {
        Self {
            key,
            api_key: api_key.to_string(),
        }
    }

    pub fn sign(&self, path: &str, body: &[u8]) -> Result<String, JwtError> {
        // tracing::debug!("signing path:'{}' hasBody:{}", path, body.is_some());
        let header = Header::new(Algorithm::RS256);
        let claims = Claims::new(path, &self.api_key, body);
        let msg = jsonwebtoken::encode(&header, &claims, &self.key)?;
        Ok(format!("Bearer {msg}"))
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}

#[derive(Debug, Deserialize, Serialize)]
/// JWT Claims as specified in [signing](https://docs.fireblocks.com/api/#signing-a-request)
struct Claims<'a> {
    /// The URI part of the request (e.g., /v1/transactions)
    uri: &'a str,
    /// Constantly increasing number. Usually, a timestamp can be used.
    nonce: u64,
    /// The time at which the JWT was issued, in seconds since Epoch.
    iat: u64,
    /// The expiration time on and after which the JWT must not be accepted for
    /// processing, in seconds since Epoch. Must be less than iat+30sec.
    exp: u64,
    /// The API key
    sub: &'a str,
    #[serde(rename = "bodyHash")]
    /// Hex-encoded SHA-256 hash of the raw HTTP request body.
    body_hash: String,
}

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("Could not serialize JWT body: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Could not create JWT time: {0}")]
    Time(#[from] std::time::SystemTimeError),
    #[error(transparent)]
    Jwt(#[from] jwterrors::Error),
    #[error(transparent)]
    TryFrom(#[from] std::num::TryFromIntError),
}

trait HexString {
    fn to_hex_string(&self) -> String;
}

impl HexString for Vec<u8> {
    #[allow(clippy::format_collect)]
    fn to_hex_string(&self) -> String {
        self.iter().map(|byte| format!("{byte:02x}")).collect()
    }
}

impl<'a> Claims<'a> {
    fn new(uri: &'a str, sub: &'a str, body: &[u8]) -> Self {
        // use millisecond precision to ensure that it's not reused
        let now = u64::try_from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis(),
        )
        .unwrap_or_default();
        let mut rng = rand::rng();
        let nonce = rng.random::<u64>();
        let now = now / 1000;

        let body_hash = {
            let mut digest = Sha256::new();
            digest.update(body);
            digest.finalize().to_vec()
        };

        Self {
            uri,
            sub,
            body_hash: body_hash.to_hex_string(),
            nonce,
            iat: now,
            exp: now + EXPIRY,
        }
    }
}
