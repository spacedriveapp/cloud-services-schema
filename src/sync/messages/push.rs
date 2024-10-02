use crate::{auth::AccessToken, devices, sync::groups};

use crate::sync::KeyHash;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub group_pub_id: groups::PubId,
	pub device_pub_id: devices::PubId,
	pub operations_count: u32,
	pub key_hash: KeyHash,
	pub start_time: DateTime<Utc>,
	pub end_time: DateTime<Utc>,
	pub expected_blob_size: u64,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpdateKind {
	Ping,
	End,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestUpdate(pub UpdateKind);

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseKind {
	SinglePresignedUrl(Url),
	ManyPresignedUrls(Vec<Url>),
	Pong,
	End,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response(pub ResponseKind);
