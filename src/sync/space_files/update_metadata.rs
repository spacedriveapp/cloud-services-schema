use crate::{auth::AccessToken, devices, sync::groups};

use serde::{Deserialize, Serialize};

use super::{FilePathPubId, ObjectPubId};

#[derive(Debug, Serialize, Deserialize, specta::Type)]
#[specta(rename = "SpaceFileUpdateMetadataRequest")]
pub struct Request {
	pub access_token: AccessToken,
	pub object_pub_id: ObjectPubId,
	pub file_path_pub_id: FilePathPubId,
	pub device_pub_id: devices::PubId,
	pub group_pub_id: groups::PubId,
	pub name: String,
	pub mime_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response;
