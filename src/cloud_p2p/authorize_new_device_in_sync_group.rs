use crate::{devices, sync::groups};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub sync_group: groups::GroupWithLibraryAndDevices,
	pub asking_device: devices::Device,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	pub authorizor_device: devices::Device,
	pub keys: Vec<Vec<u8>>,
}
