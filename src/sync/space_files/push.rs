use crate::{auth::AccessToken, devices, sync::groups};

use serde::{Deserialize, Serialize};

use super::{FilePathPubId, PubId, SpaceFileEncryptedChunk};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub file_path_pub_id: FilePathPubId,
	pub device_pub_id: devices::PubId,
	pub group_pub_id: groups::PubId,
	pub name: String,
	pub mime_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestUpdate(pub SpaceFileEncryptedChunk);

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
	Uploaded,
	Relinked,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	pub pub_id: PubId,
	pub status: Status,
}
