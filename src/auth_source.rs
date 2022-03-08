use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AuthSource {
    pub name: String,
    pub issuer: String,
    pub secrete: String,
    pub algorithm: String,
    pub digits: usize,
    pub period: usize,
}

impl Default for AuthSource {
    fn default() -> Self {
        AuthSource {
            name: "".into(),
            issuer: "".into(),
            secrete: "".into(),
            algorithm: "SHA1".into(),
            digits: 6,
            period: 30,
        }
    }
}
