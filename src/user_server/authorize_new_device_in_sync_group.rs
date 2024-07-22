use crate::{auth::ServerSignedToken, devices, sync::groups};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub server_signature_token: ServerSignedToken,
	pub sync_group: groups::Group,
	pub device: devices::Device,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response;
