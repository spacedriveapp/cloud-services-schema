use crate::{
	auth::{AccessToken, DevicePublicKey},
	devices,
};

use serde::{Deserialize, Serialize};

use super::PubId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
	pub current_device_pub_id: devices::PubId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response(pub Vec<(devices::PubId, DevicePublicKey)>);
