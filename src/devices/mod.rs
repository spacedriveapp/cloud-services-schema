use crate::auth::DevicePublicKey;

use chrono::{DateTime, Utc};
use iroh_base::ticket::NodeTicket;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

crate::declare!(
	parent = crate,
	rpc = [get, list, update, delete],
	bidirectional_stream = [register, hello]
);

#[derive(Debug, Serialize, Deserialize, derive_more::Display)]
#[serde(transparent)]
pub struct PubId(pub Uuid);

#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceOS {
	Linux,
	Windows,
	MacOS,
	IOS,
	Android,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
	pub pub_id: PubId,
	pub name: String,
	pub os: DeviceOS,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub storage_size: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub public_key: Option<DevicePublicKey>,
	pub connection_id: NodeTicket,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

pub mod delete;
pub mod get;
pub mod hello;
pub mod list;
pub mod register;
pub mod update;
