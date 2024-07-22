use crate::{auth::P2PSignedToken, devices, sync::groups};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub p2p_signed_token: P2PSignedToken,
	pub sync_group: groups::Group,
	pub device: devices::Device,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	pub authorizor_device: devices::Device,
}
