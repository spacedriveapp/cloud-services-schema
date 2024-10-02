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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[specta(rename = "CloudSyncMessagesCollection")]
pub struct MessagesCollection {
	pub original_device_pub_id: devices::PubId,
	pub start_time: DateTime<Utc>,
	pub end_time: DateTime<Utc>,
	pub operations_count: u32,
	pub key_hash: KeyHash,
	#[specta(type = String)]
	pub signed_download_link: Url,
}

pub mod delete_older;
pub mod get_latest_time;
pub mod pull;
pub mod push;
