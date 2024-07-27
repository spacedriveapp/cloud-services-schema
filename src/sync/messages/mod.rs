use crate::devices;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

crate::declare! {
	parent = super,
	rpc = [delete_older],
	client_stream = [push],
	server_stream = [pull],
}

crate::need_auth!(delete_older, pull, push);

use super::KeyHash;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagesCollection {
	original_device_pub_id: Option<devices::PubId>,
	start_time: DateTime<Utc>,
	end_time: DateTime<Utc>,
	operations_count: u32,
	key_hash: KeyHash,
	signed_download_link: Url,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MessagesCollectionEncryptedChunk(pub Vec<u8>);

pub mod delete_older;
pub mod pull;
pub mod push;
