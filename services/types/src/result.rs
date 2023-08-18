use eyre::Result;
use marine_rs_sdk::marine;
use serde::{Deserialize, Serialize};

#[marine]
#[derive(Deserialize, Debug, Clone)]
pub struct TrieResult {
  pub success: bool,
  pub result: String,
}

impl From<Result<String>> for TrieResult {
  fn from(result: Result<String>) -> Self {

    match result {
      Ok(data) => {
        let raw_data: String = serde_json::from_str(&data).unwrap();

        // println!("raw_data: {:?}", raw_data);

        serde_json::from_str(&raw_data).unwrap_or(TrieResult { success: false, result: "".to_string() })
      },
      _ => Self { success: false, result: "".to_string() },
    }
  }
}

#[marine]
#[derive(Debug)]
pub struct FdbResult {
    pub transaction_hash: String,
}
