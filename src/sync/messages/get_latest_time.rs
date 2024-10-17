use crate::{auth::AccessToken, devices, sync::groups};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Kind {
	ForCurrentDevice(devices::PubId),
	ForAnyDevice,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub group_pub_id: groups::PubId,
	pub kind: Kind,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
	pub latest_time: DateTime<Utc>,
	pub latest_device_pub_id: devices::PubId,
}
