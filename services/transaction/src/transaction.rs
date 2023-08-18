use serde::{Deserialize, Serialize};
use marine_rs_sdk::marine;
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrieTransaction {
  pub transaction: Transaction,
  pub receipt: Option<TransactionReceipt>,
}

#[marine]
#[derive(Debug, Default, Clone)]
pub struct TransactionRequest {
    // pub data_key: String,
    pub program_id: String,
    pub alias: String,
    pub public_key: String,
    pub signature: String,
    pub data: String,
    pub method: String,
    pub chain_id: String,
    pub token_address: String,
    pub token_id: String,
    pub version: String,
    pub mcdata: String,
}

#[marine]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: String,
    pub method: String,
    pub program_id: String,
    pub data_key: String,
    pub data: String,
    pub public_key: String,
    pub alias: String,
    pub timestamp: u64,
    pub chain_id: String,
    pub token_address: String,
    pub token_id: String,
    pub version: String,
    pub mcdata: String,
    pub status: i64,
}

#[marine]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TransactionReceipt {
    pub hash: String,
    pub program_id: String,
    pub status: i64,
    pub timestamp: u64,
    pub error_text: String,
    pub data: String,
}

impl Transaction {
  pub fn new(
      program_id: String,
      data_key: String,
      data: String,
      public_key: String,
      alias: String,
      timestamp: u64,
      method: String,
      chain_id: String,
      token_address: String,
      token_id: String,
      version: String,
      mcdata: String,
      status: i64,
      previous_data: String,
  ) -> Self {
      let hash = Self::generate_hash(
          program_id.clone(),
          data_key.clone(),
          data.clone(),
          public_key.clone(),
          alias.clone(),
          method.clone(),
          // chain_id.clone(),
          // token_address.clone(),
          // token_id.clone(),
          version.clone(),
          mcdata.clone(),
          previous_data,
      );

      Self {
          hash,
          method,
          program_id,
          data_key,
          data,
          public_key,
          alias,
          timestamp,
          chain_id,
          token_address,
          token_id,
          version,
          mcdata,
          status,
      }
  }

  /**
   * Generating new transaction hash
   * Using the formula hash(tx datas + previous_data)
   * This hash would only prevent replay attach if the transaction content is similar from the current content.
   * A good example is health/mana level - There will be multiple duplicate data
   */
  pub fn generate_hash(
      program_id: String,
      data_key: String,
      data: String,
      public_key: String,
      alias: String,
      method: String,
      // chain_id: String,
      // token_address: String,
      // token_id: String,
      version: String,
      mcdata: String,
      previous_content: String,
  ) -> String {
      let mut hasher = Sha256::new();
      hasher.update(
          format!(
              "{}{}{}{}{}{}{}{}{}",
              program_id,
              data_key,
              data,
              public_key,
              alias,
              method,
              // chain_id,
              // token_address,
              // token_id,
              version,
              mcdata,
              previous_content
          )
          .as_bytes(),
      );
      bs58::encode(hasher.finalize()).into_string()
  }
  
}