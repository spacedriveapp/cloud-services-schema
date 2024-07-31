use crate::{devices, sync::groups};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub sync_group: groups::Group,
	pub device: devices::Device,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response;
