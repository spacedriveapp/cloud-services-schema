use crate::{auth::AccessToken, devices};

use serde::{Deserialize, Serialize};

use super::PubId;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
	pub current_device_pub_id: devices::PubId,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response;
