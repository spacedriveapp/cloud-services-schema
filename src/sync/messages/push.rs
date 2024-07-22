use crate::{auth::AccessToken, devices, sync::groups};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::MessagesCollectionEncryptedChunk;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub group_pub_id: groups::PubId,
	pub device_pub_id: devices::PubId,
	pub operations_count: u32,
	pub start_time: DateTime<Utc>,
	pub end_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestUpdate(pub MessagesCollectionEncryptedChunk);

#[derive(Debug, Serialize, Deserialize)]
pub struct Response;
