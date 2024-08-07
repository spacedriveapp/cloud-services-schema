use crate::{auth::AccessToken, devices};

use serde::{Deserialize, Serialize};

use super::{KeyHash, PubId};

#[derive(Debug, Serialize, Deserialize, specta::Type)]
#[specta(rename = "SyncGroupRemoveDeviceRequest")]
pub struct Request {
	pub access_token: AccessToken,
	pub group_pub_id: PubId,
	pub new_key_hash: KeyHash,
	pub current_device_pub_id: devices::PubId,
	pub to_remove_device_pub_id: devices::PubId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response;
