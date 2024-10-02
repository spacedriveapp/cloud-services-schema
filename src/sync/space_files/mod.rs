use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use super::KeyHash;

crate::declare! {
	parent = super,
	rpc = [delete, pull],
	server_stream = [list],
	bidirectional_stream = [push, update],
}

crate::need_auth!(delete, list, pull, push, update);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(transparent)]
#[specta(transparent, rename = "CloudObjectPubId")]
pub struct ObjectPubId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(transparent)]
#[specta(transparent, rename = "CloudFilePathPubId")]
pub struct FilePathPubId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[specta(rename = "CloudSpaceFile")]
pub struct SpaceFile {
	pub object_pub_id: ObjectPubId,
	pub file_path_pub_id: FilePathPubId,
	pub key_hash: KeyHash,
	pub size: u64,
	pub uploaded_at: DateTime<Utc>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	#[specta(type = String)]
	pub presigned_download_link: Option<Url>,
}

pub mod delete;
pub mod list;
pub mod pull;
pub mod push;
pub mod update;
