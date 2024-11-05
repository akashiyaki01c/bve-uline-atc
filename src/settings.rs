use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct Settings {
    atc_type: String
}