use crate::{auth::AccessToken, devices, sync::groups};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::MessagesCollection;

pub const BATCH_SIZE: u32 = 100;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub group_pub_id: groups::PubId,
	pub current_device_pub_id: devices::PubId,
	pub start_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response(pub Vec<MessagesCollection>);
