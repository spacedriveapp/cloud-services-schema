use crate::{auth::AccessToken, devices, libraries};

use serde::{Deserialize, Serialize};

use super::PubId;

#[derive(Debug, Serialize, Deserialize, specta::Type)]
#[specta(rename = "LocationCreateRequest")]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
	pub name: String,
	pub library_pub_id: libraries::PubId,
	pub device_pub_id: devices::PubId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response;
