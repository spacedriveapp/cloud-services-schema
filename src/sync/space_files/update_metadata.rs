use crate::{auth::AccessToken, devices, sync::groups};

use serde::{Deserialize, Serialize};

use super::PubId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
	pub device_pub_id: devices::PubId,
	pub group_pub_id: groups::PubId,
	pub name: String,
	pub mime_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response;
