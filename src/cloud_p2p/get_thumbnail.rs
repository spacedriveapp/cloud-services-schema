use crate::{devices, libraries};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
	pub cas_id: String,
	pub device_pub_id: devices::PubId,
	pub library_pub_id: libraries::PubId,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
	pub thumbnail: Option<Vec<u8>>,
}
