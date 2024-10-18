use crate::{auth::AccessToken, devices, sync::groups};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::MessagesCollection;

pub const BATCH_SIZE: u32 = 10;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub group_pub_id: groups::PubId,
	pub current_device_pub_id: devices::PubId,
	pub start_time_per_device: Vec<(devices::PubId, DateTime<Utc>)>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response(pub Vec<MessagesCollection>);
