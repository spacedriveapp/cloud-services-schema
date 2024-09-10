use crate::auth::AccessToken;

use serde::{Deserialize, Serialize};

use super::PubId;

#[derive(Debug, Serialize, Deserialize, specta::Type)]
#[specta(rename = "DeviceUpdateRequest")]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
	pub name: String,
	pub storage_size: u64,
	pub used_storage: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response;
