use crate::{auth::AccessToken, devices};

use serde::{Deserialize, Serialize};

use super::PubId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub group_pub_id: PubId,
	pub authorized_device_pub_id: devices::PubId,
	pub authorizor_device_pub_id: devices::PubId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response;
