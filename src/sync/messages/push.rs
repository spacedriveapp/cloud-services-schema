use crate::{auth::AccessToken, devices, sync::groups, sync::KeyHash};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub group_pub_id: groups::PubId,
	pub device_pub_id: devices::PubId,
	pub operations_count: u32,
	pub key_hash: KeyHash,
	pub time_range: (DateTime<Utc>, DateTime<Utc>),
	pub encrypted_messages: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response;
