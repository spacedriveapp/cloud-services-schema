use crate::{auth::AccessToken, sync::groups};

use serde::{Deserialize, Serialize};

use super::{FilePathPubId, ObjectPubId};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub object_pub_id: ObjectPubId,
	pub file_path_pub_id: FilePathPubId,
	pub group_pub_id: groups::PubId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response;
