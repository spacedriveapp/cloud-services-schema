use crate::devices;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

crate::declare! {
	parent = super,
	rpc = [delete_older],
	server_stream = [pull],
	bidirectional_stream = [push],
}

crate::need_auth!(delete_older, pull, push);

use super::KeyHash;

#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub struct MessagesCollection {
	original_device_pub_id: Option<devices::PubId>,
	start_time: DateTime<Utc>,
	end_time: DateTime<Utc>,
	operations_count: u32,
	key_hash: KeyHash,
	#[specta(type = String)]
	signed_download_link: Url,
}

pub mod delete_older;
pub mod pull;
pub mod push;
