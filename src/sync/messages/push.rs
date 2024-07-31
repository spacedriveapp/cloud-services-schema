use crate::{auth::AccessToken, devices, sync::groups};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub group_pub_id: groups::PubId,
	pub device_pub_id: devices::PubId,
	pub operations_count: u32,
	pub start_time: DateTime<Utc>,
	pub end_time: DateTime<Utc>,
	pub expected_blob_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UpdateKind {
	Ping,
	End,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestUpdate(pub UpdateKind);

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseKind {
	SinglePresignedUrl(Url),
	ManyPresignedUrls(Vec<Url>),
	Pong,
	End,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response(pub ResponseKind);
