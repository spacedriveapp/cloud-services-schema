use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use super::KeyHash;

crate::declare! {
	parent = super,
	rpc = [delete, pull, update_metadata],
	client_stream = [push, update],
	server_stream = [list],
}

crate::need_auth!(delete, list, pull, push, update, update_metadata);

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectPubId(pub Uuid);

#[derive(Debug, Serialize, Deserialize)]
pub struct FilePathPubId(pub Uuid);

#[derive(Debug, Serialize, Deserialize)]
pub struct SpaceFile {
	pub object_pub_id: ObjectPubId,
	pub file_path_pub_id: FilePathPubId,
	pub key_hash: KeyHash,
	pub name: String,
	pub size: u64,
	pub mime_type: String,
	pub uploaded_at: DateTime<Utc>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub signed_download_link: Option<Url>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SpaceFileEncryptedChunk(pub Vec<u8>);

pub mod delete;
pub mod list;
pub mod pull;
pub mod push;
pub mod update;
pub mod update_metadata;
