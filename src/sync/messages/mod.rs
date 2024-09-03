use crate::devices;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

crate::declare! {
	parent = super,
	rpc = [delete_older, get_latest_time],
	server_stream = [pull],
	bidirectional_stream = [push],
}

crate::need_auth!(delete_older, pull, push, get_latest_time);

use super::KeyHash;

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct MessagesCollection {
	original_device_pub_id: devices::PubId,
	start_time: DateTime<Utc>,
	end_time: DateTime<Utc>,
	operations_count: u32,
	key_hash: KeyHash,
	#[specta(type = String)]
	signed_download_link: Url,
}

pub mod delete_older;
pub mod get_latest_time;
pub mod pull;
pub mod push;
