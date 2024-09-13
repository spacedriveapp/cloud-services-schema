use crate::{devices, sync::groups, libraries};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
	pub sync_group: groups::GroupWithLibraryAndDevices,
	pub asking_device: devices::Device,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	pub authorizor_device: devices::Device,
	pub keys: Vec<Vec<u8>>,
	pub library_pub_id: libraries::PubId,
	pub library_name: String,
	pub library_description: Option<String>
}
