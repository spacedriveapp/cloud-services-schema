use crate::devices;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
	pub cas_id: String,
	pub device_pub_id: devices::PubId,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
	pub thumbnail: Option<Vec<u8>>,
}
