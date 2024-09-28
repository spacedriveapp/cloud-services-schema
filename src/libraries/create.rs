use crate::{auth::AccessToken, devices};

use serde::{Deserialize, Serialize};

use super::PubId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
	pub name: String,
	pub device_pub_id: devices::PubId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response;
