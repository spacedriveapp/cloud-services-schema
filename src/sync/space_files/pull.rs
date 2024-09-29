use crate::{auth::AccessToken, devices, sync::groups};

use serde::{Deserialize, Serialize};

use super::{FilePathPubId, ObjectPubId, SpaceFile};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub object_pub_id: ObjectPubId,
	pub file_path_pub_id: FilePathPubId,
	pub name: String,
	pub group_pub_id: groups::PubId,
	pub current_device_pub_id: devices::PubId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response(pub SpaceFile);
