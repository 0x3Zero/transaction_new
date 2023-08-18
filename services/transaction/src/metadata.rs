use marine_rs_sdk::marine;
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[marine]
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Metadata {
    pub hash: String,
    pub data_key: String,
    pub program_id: String,
    pub alias: String,
    pub cid: String,
    pub chain_id: String,
    pub token_address: String,
    pub token_id: String,
    pub version: String,
    pub public_key: String,
    pub loose: bool,
}

impl Metadata {
    pub fn new(
        data_key: String,
        program_id: String,
        alias: String,
        cid: String,
        public_key: String,
        chain_id: String,
        token_address: String,
        token_id: String,
        version: String,
        loose: bool,
    ) -> Self {
        let hash = Self::generate_hash(
            // chain_id.clone(),
            // token_address.clone(),
            // token_id.clone(),
            data_key.clone(),
            version.clone(),
            alias.clone(),
            public_key.clone(),
        );

        let data_key = Self::generate_data_key(
          chain_id.clone(), 
          token_address.clone(), 
          token_id.clone(),
        );

        Self {
            hash,
            data_key,
            program_id,
            alias,
            cid,
            chain_id,
            token_address,
            token_id,
            version,
            public_key,
            loose,
        }
    }

    pub fn generate_hash(
        // chain_id: String,
        // token_address: String,
        // token_id: String,
        data_key: String,
        version: String,
        alias: String,
        public_key: String,
    ) -> String {
        // let data_key = Self::generate_data_key(chain_id, token_address, token_id);

        format!("{}{}{}{}",
          data_key,
          version,
          alias,
          public_key,
        )
    }

    pub fn generate_data_key(
      chain_id: String,
      token_address: String,
      token_id: String,
  ) -> String {
      let mut hasher = Sha256::new();
      hasher.update(
          format!(
              "{}{}{}",
              chain_id, token_address, token_id,
          )
          .as_bytes(),
      );
      bs58::encode(hasher.finalize()).into_string()
  }
    
}