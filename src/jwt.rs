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
// Default RSA private key for development/testing purposes
const DEFAULT_RSA_PRIVATE_KEY: &str = r#"-----BEGIN PRIVATE KEY-----
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQDDlZwIUVmNEuOC
3qx9Obj56/5j9m2rlP7TRCVFKkLCABYMkeJN+hinJhvCkedxN3jqdZUzK0VvImYG
kvOxxyiUmdqEkMUK4ZDmnpfzTI4q3dHvMu9QSd5tlAdAaaFVF9vkSisXBVW0u3Jy
MtAHBPYmbq0n9W1KPz56i4aEItLNjFnYjqu8IGh1RMIM9zcp83JIoE2ggYBQpHSa
ZHwU6UozonrEgTf7UQYv7PrWSXODJJ6d1eb6dGrVKFhAFppwAtIKFA3EWiFQgiHJ
UdeMAKB0bc2Lt7QYSe8NfQ0Hn50XrZfcdigwUUzytaGAaWPQSMP+/uIGqI5CfV3D
z6s+uFpDAgMBAAECggEACn1st/l8/xcDQtKkl7bP1/+zTjM6YJiGLliaUyJYIEWW
6Set5pbCWbyugnoG2sip4JFb5jms6LAEntek4GUZJRc3ZCLLnrDIb8YTykXR3jS1
fMnTkg/UMTZeSTMhJwo3jf/4Xay4kw/2rG9TPv0iIp+PA/Si3veTZ0kydsXTXpjP
WQknegze2qJrW35Lj4b5LNTe++p3+Gcv1jyzOkr9jX3FG7OlcdGsg0FH2S7N5Rd/
YHvKO6LUxRk+hvjG+XV2tYlrGm3Hc1M9u3qSLlvx0IABN0d5yNQHqDueIfIjCIXj
GihPSY2Rp5G32slAAw54ZxGIB+i/ggdIyqnjOVeNoQKBgQD4+ZAK+Ma/ZHH/M0/o
Llw4ICKIInjnqcgcEyhbL0la1/PzF2gdhgYAvvVNzvaHl/LtTS+2W2yJWuzzUstC
lgK5n8+AySmEg2kDM1fhYgkZG8uRRNpN7Lf8tQc5ZmC0XNKJjqOrHpxPJOrWSYSP
fKcYQoRe0ty/zt+Vm2KQ7+ga8QKBgQDJGmNQx7HdijZSQqC/9MdRL9gFq/TwtqdO
vPHhUPYa4DXOSaERUgjLQTSgr7vdbtfuWTsmP4uHdrwvrAQLuexF7yywPg6b/hAV
k0Jz8ps9UV0jyPaFZHbkH9nL8QxwXZC5FrtxD+WN7QJn1irC+Hwz0VAeJ25VkaNB
3KH8bjhAcwKBgQDbz1lAlorBhuiuKpstnWCFnLlf/y9HODoXr0I93u2ganBR+iRN
jHnYRr9DxqkY5SnwH+kz0ltsmP4BhOM3EkGtqE4GoZMqDuqzjKzVqzvlEwkSY/to
OMnnLdwG7UALfLfUAj61YP0XUrySG64REDhlzrQXE4sZPIdhCiJnW6dLwQKBgA+x
SXKyQoThFiARJu4TscH6E8dNhc2K0z9nqxBD+xBZ0pkIUdNCLYF+0xZ+4BAFaEAn
ImB3sPGfKEwoBiDMH03NvhT6orU1fBfS5+qYUdjVEomKUwJRtp1ShvJNGwVhp7tp
tLK75NPQXNGxeqDANyDsAm538TooJS8sgk9qYmVVAoGBAKWa2FZqQ5KX/8XNKf74
jEPCF1kvsYtTk1mCSf5mnW0pLa6LZ5oiP1tEs/7MVMYSNRFFGmrBpbOCRCOBwVpn
6FEj7Wk6LKikJpGPITG+IKCs+yIreYp5aw7HUUBuF+meqhpka6B8+cFnyoIVdxnW
ZfPTMjUf7hKMYnp7Cf5KCdC1
-----END PRIVATE KEY-----"#;

#[derive(Clone)]
pub struct JwtSigner {
    key: EncodingKey,
    api_key: String,
}

impl Default for JwtSigner {
    fn default() -> Self {
        let key = EncodingKey::from_rsa_pem(DEFAULT_RSA_PRIVATE_KEY.as_bytes())
            .expect("Failed to create default RSA encoding key");

        Self {
            key,
            api_key: String::new(),
        }
    }
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
