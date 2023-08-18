#![allow(improper_ctypes)]

mod transaction;
mod error;
mod defaults;
mod utils;
mod meta_contract;
mod metadata;

use error::ServiceError::{
  self, InvalidMethod, InvalidOwner, InvalidSignature, NoEncryptionType, NoProgramId,
  NotSupportedEncryptionType, RecordFound,
};
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;
use meta_contract::MetaContract;
use meta_contract::TxContract;
use metadata::Metadata;
use serde_json::Error;
use serde_json::Value;
use types::*;
use transaction::*;
use defaults::*;
use eyre::Result;
use utils::hasher;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

module_manifest!();

#[macro_use]
extern crate fstrings;

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

#[marine]
pub fn publish(
    // data_key: String,
    // method: String,
    // alias: String,
    // public_key: String,
    // program_id: String,
    // signature: String,
    // data: String,
    // mcdata: String,
    // chain_id: String,
    // token_address: String,
    // token_id: String,
    // version: String,
    tx_request: TransactionRequest,
) -> FdbResult {
    let mut program_id = tx_request.program_id.clone();
    let mut data_key = "".to_string();
    let mut error: Option<ServiceError> = None;
    let mut content = "".to_string();
    // let storage = get_storage();

    if error.is_none() {
        if tx_request.method != METHOD_CONTRACT
            && tx_request.method != METHOD_METADATA
            && tx_request.method != METHOD_CLONE
            && tx_request.method != METHOD_CRON
        {
            error = Some(InvalidMethod(f!("invalid method: {tx_request.method}")));
        }
    }

    let enc_verify = get_public_key_type(tx_request.public_key.clone().as_str());
    if enc_verify.len() <= 0 {
        error = Some(ServiceError::InvalidEncryption(tx_request.public_key.clone()));
    }

    if error.is_none() {
      if tx_request.method.clone() == METHOD_CONTRACT {
          // to generate unique program_id
          // program_id = hasher(f!("{}{}", tx_request.data.clone(), tx_request.public_key.clone()));
          let parse_data: Result<TxContract, Error> = serde_json::from_str(&tx_request.data.clone());

          match parse_data {
            Ok(data) => {
              program_id = MetaContract::generate_program_id(data.cid.clone(), tx_request.public_key.clone());
              content = program_id.clone();

              let result = filter_trie("metacontract".to_string(), program_id.clone(), None);
              
              // Check if the metacontract exist

              if result.success {
                error = Some(ServiceError::RecordFound(program_id.clone()));
                let metacontract: Result<Vec<MetaContract>, Error> = serde_json::from_str(&result.result);

                match metacontract {
                  Ok(mc) => {
                    let d = mc.get(0).unwrap().clone();
                    content = d.program_id;
                  },
                  Err(e) => error = Some(ServiceError::InvalidDataFormatForMethodType(program_id.clone())),
                }
              } 
            },
            Err(e) => error = Some(ServiceError::InvalidDataFormatForMethodType(program_id.clone())),
          }
      } 
      else if tx_request.method.clone() == METHOD_METADATA {
          if program_id.clone().is_empty() {
              error = Some(NoProgramId());
          }

          if (error.is_none()) {
              
            let mc_result = filter_trie("metacontract".to_string(), program_id.clone(), None);

            data_key = Metadata::generate_data_key(
              tx_request.chain_id.clone(), 
              tx_request.token_address.clone(), 
              tx_request.token_id.clone(),
            );

            if mc_result.success {
              let hash = Metadata::generate_hash(
                // tx_request.chain_id.clone(), 
                // tx_request.token_address.clone(), 
                // tx_request.token_id.clone(), 
                data_key.clone(),
                hasher(tx_request.version.clone()), 
                hasher(tx_request.alias.clone()), 
                tx_request.public_key.clone(),
              );

              let result = filter_trie("metadata".to_string(), "hash".to_string(), Some(hash));

              if result.success {
                let metadata: Result<Vec<Metadata>, _> = serde_json::from_str(&result.result);

                match metadata {
                  Ok(data) => {
                    let d = data.get(0).unwrap().clone();
                    if d.public_key != tx_request.public_key {
                      error = Some(InvalidOwner(f!("not owner of data_key: {tx_request.public_key}")));
                    }
                    content = d.cid;
                  },
                  _ => (),
                }
              }
            } else {
              error = Some(NoProgramId());
            }
          }
      }
    }

    // if error.is_none() {
    //     if method.clone() == METHOD_METADATA {
    //         if program_id.clone().is_empty() {
    //             error = Some(NoProgramId());
    //         }

    //         if (error.is_none()) {
    //             let result = storage.get_owner_metadata(
    //                 data_key.clone(),
    //                 program_id.clone(),
    //                 public_key.clone(),
    //                 alias.clone(),
    //                 version.clone(),
    //             );

    //             match result {
    //                 Ok(metadata) => {
    //                     if metadata.public_key != public_key {
    //                         error = Some(InvalidOwner(f!("not owner of data_key: {public_key}")));
    //                     }

    //                     content = metadata.cid;
    //                 }
    //                 Err(ServiceError::RecordNotFound(_)) => {}
    //                 Err(e) => error = Some(e),
    //             }
    //         }
    //     } else if method.clone() == METHOD_CONTRACT {
    //         // to generate unique program_id
    //         program_id = hasher(f!("{}{}", data.clone(), public_key.clone()));
    //         // Check if the
    //         let result = storage.get_meta_contract(program_id.clone());
    //         match result {
    //             Ok(metacontract) => {
    //                 error = Some(ServiceError::RecordFound(program_id.clone()));
    //                 content = metacontract.program_id;
    //             }
    //             Err(e) => error = Some(e),
    //         }
    //         content = program_id.clone();
    //     } else if method.clone() == METHOD_CLONE {
    //         let data_clone_result: Result<DataTypeClone, serde_json::Error> =
    //             serde_json::from_str(&data.clone());

    //         match data_clone_result {
    //             Ok(data_clone) => {
    //                 let origin_metadata_result = storage.get_owner_metadata(
    //                     data_clone.origin_data_key.clone(),
    //                     data_clone.origin_program_id.clone(),
    //                     data_clone.origin_public_key.clone(),
    //                     data_clone.origin_alias.clone(),
    //                     data_clone.origin_version.clone(),
    //                 );

    //                 match origin_metadata_result {
    //                     Ok(_) => {}
    //                     Err(e) => error = Some(e),
    //                 }

    //                 if error.is_none() {
    //                     let new_metadata_result = storage.get_owner_metadata(
    //                         data_key.clone(),
    //                         data_clone.origin_program_id.clone(),
    //                         data_clone.origin_public_key.clone(),
    //                         data_clone.origin_alias.clone(),
    //                         data_clone.origin_version.clone(),
    //                     );

    //                     match new_metadata_result {
    //                         Ok(_) => error = Some(RecordFound(data_key.clone())),
    //                         Err(ServiceError::RecordNotFound(_)) => {}
    //                         Err(e) => error = Some(e),
    //                     }
    //                 }
    //             }
    //             Err(_) => {
    //                 error = Some(ServiceError::InvalidDataFormatForMethodType(method.clone()))
    //             }
    //         }
    //     } else if method.clone() == METHOD_CRON {
    //         let cron_result: Result<SerdeCron, serde_json::Error> = serde_json::from_str(&data);

    //         match cron_result {
    //             Ok(serde_cron) => {
    //                 if serde_cron.action == CRON_ACTION_CREATE {
    //                     if serde_cron.address.len() <= 0
    //                         || serde_cron.chain.len() <= 0
    //                         || serde_cron.topic.len() <= 0
    //                         || serde_cron.token_type.len() <= 0
    //                     {
    //                         error =
    //                             Some(ServiceError::InvalidDataFormatForMethodType(method.clone()))
    //                     } else {
    //                         let result = storage.search_cron(
    //                             serde_cron.address.clone(),
    //                             serde_cron.chain.clone(),
    //                             serde_cron.topic.clone(),
    //                         );

    //                         match result {
    //                             Ok(_) => {
    //                                 error = Some(RecordFound(f!(
    //                                 "{serde_cron.address} {serde_cron.chain} {serde_cron.topic}"
    //                             )))
    //                             }
    //                             Err(ServiceError::RecordNotFound(_)) => {}
    //                             Err(e) => error = Some(e),
    //                         }
    //                     }
    //                 } else {
    //                     if serde_cron.hash.is_empty()
    //                         || (serde_cron.status != CRON_STATUS_ACTIVE
    //                             && serde_cron.status != CRON_STATUS_DISABLE)
    //                     {
    //                         error =
    //                             Some(ServiceError::InvalidDataFormatForMethodType(method.clone()))
    //                     } else {
    //                         let result = storage.get_cron_by_hash(serde_cron.hash);
    //                         match result {
    //                             Ok(_) => {}
    //                             Err(e) => error = Some(e),
    //                         }
    //                     }
    //                 }
    //             }
    //             Err(e) => error = Some(ServiceError::InvalidDataFormatForMethodType(e.to_string())),
    //         }
    //     } else {
    //         error = Some(InvalidMethod(f!("{method}")));
    //     }
    // }

    if error.is_none() {
        if enc_verify.clone().is_empty() {
            error = Some(NoEncryptionType())
        } else {
            if enc_verify.clone().ne(ENCRYPTION_TYPE_SECP256K1)
                && enc_verify.clone().ne(ENCRYPTION_TYPE_ED25519)
            {
                error = Some(NotSupportedEncryptionType(enc_verify.clone()));
            }
        }
    }

    if error.is_none() {
        let v = verify(
          tx_request.public_key.clone(),
          tx_request.signature.clone(),
          tx_request.data.clone(),
          enc_verify.clone(),
        );

        if !v {
            error = Some(InvalidSignature(f!("not owner of data_key: {tx_request.public_key}")));
        }
    }

    // let cp = marine_rs_sdk::get_call_parameters();

    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    // Hash version
    let mut transaction = Transaction::new(
        program_id,
        data_key,
        tx_request.data,
        tx_request.public_key,
        tx_request.alias,
        timestamp.as_millis() as u64,
        tx_request.method,
        tx_request.chain_id,
        tx_request.token_address,
        tx_request.token_id,
        tx_request.version,
        tx_request.mcdata,
        0,
        content,
    );

    let mut receipt: Option<TransactionReceipt> = None;

    if !error.is_none() {
        transaction.status = STATUS_FAILED;

        receipt = Some(TransactionReceipt { 
          hash: transaction.hash.clone(), 
          program_id: transaction.program_id.clone(), 
          status: STATUS_FAILED, 
          timestamp: timestamp.as_millis() as u64, 
          error_text: error.unwrap().to_string(), 
          data: transaction.data.clone(), 
        })
    }

    let trie_tx = TrieTransaction {
      transaction: transaction.clone(),
      receipt,
    };

    // println!("trie_tx: {:?}", trie_tx);

    insert_tx(trie_tx);
    // println!("result: {:?}", result);

    // let _ = storage.write_transaction(transaction.clone());

    FdbResult {
        transaction_hash: transaction.hash,
    }
}

fn insert_tx(trie_tx: TrieTransaction) -> TrieResult {
  let tx = serde_json::to_string(&trie_tx).unwrap();

  let args = vec![
    "insert_tx".to_string(),
    tx
  ];
  
  let result = unwrap_mounted_binary_result(world_state(args))
        .map(|res| res.trim().to_string());
  println!("result: {:?}", result);
  result.into()
}

#[marine]
pub fn publish_batch(txs: Vec<TransactionRequest>) -> Vec<FdbResult> {
    let mut results: Vec<FdbResult> = vec![];

    for tx in txs {
        let result = publish(
            // tx.data_key,
            tx
        );

        results.push(result);
    }
    results
}

fn filter_trie(trie_key: String, filter_key: String, filters: Option<String>) -> TrieResult {
  let mut args = vec![
      "filter_trie".to_string(),
      trie_key,
      filter_key,
  ];

  if filters.is_some() {
    args.push(filters.unwrap());
  }
  
  unwrap_mounted_binary_result(world_state(args))
        .map(|res| res.trim().to_string())
        .into()
}

#[marine]
pub fn get_tx() -> TrieResult {
  let args = vec!["get_pending_tx".to_string()];
  
  unwrap_mounted_binary_result(world_state(args))
        .map(|res| res.trim().to_string()).into()

}

#[marine]
pub fn test_print() {
  println!("test print");
}

#[marine]
pub fn test_filter() -> TrieResult {
  let s = json!([
    {
      "method": "method2"
    },
    {
      "alias": "alias2"
    }
  ]);
  let filters = serde_json::to_string(&s).unwrap();
  let result = filter_trie("tx".to_string(), "123".to_string(), Some(filters));

  result
}

fn unwrap_mounted_binary_result(result: MountedBinaryResult) -> Result<String> {
  // println!("binary result: {:?}", result);
  result
      .into_std()
      .ok_or(eyre::eyre!(
          "stdout or stderr contains non valid UTF8 string"
      ))?
      .map_err(|e| eyre::eyre!("cli call failed: {}", e))
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    /// Execute provided cmd as a parameters of ipfs cli, return result.
    pub fn world_state(cmd: Vec<String>) -> MountedBinaryResult;

}

#[marine]
#[link(wasm_import_module = "crypto")]
extern "C" {
    #[link_name = "verify"]
    pub fn verify(public_key: String, signature: String, message: String, enc: String) -> bool;

    #[link_name = "get_public_key_type"]
    pub fn get_public_key_type(public_key: &str) -> String;
}