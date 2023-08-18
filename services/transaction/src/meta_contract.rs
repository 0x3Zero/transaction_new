use marine_rs_sdk::marine;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[marine]
#[derive(Debug, Default, Clone, Deserialize)]
pub struct MetaContract {
    pub program_id: String,
    pub public_key: String,
    pub cid: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct TxContract {
    // pub program_id: String,
    pub cid: String,
}

impl MetaContract {
  pub fn generate_program_id(
      cid: String,
      public_key: String,
  ) -> String {
      let mut hasher = Sha256::new();
      hasher.update(
          format!(
              "{}{}",
              cid, public_key,
          )
          .as_bytes(),
      );
      bs58::encode(hasher.finalize()).into_string()
  }
}