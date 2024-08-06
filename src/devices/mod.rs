use chrono::{DateTime, Utc};
use iroh_base::key::NodeId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

crate::declare! {
	parent = crate,
	rpc = [get, list, update, delete],
	bidirectional_stream = [register, hello],
}

crate::need_auth!(get, list, update, delete, register, hello);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, derive_more::Display, specta::Type)]
#[serde(transparent)]
pub struct PubId(pub Uuid);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, specta::Type)]
pub enum DeviceOS {
	Linux,
	Windows,
	MacOS,
	IOS,
	Android,
}

#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub struct Device {
	pub pub_id: PubId,
	pub name: String,
	pub os: DeviceOS,
	pub storage_size: u64,
	#[specta(type = String)]
	pub connection_id: NodeId,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

pub mod delete;
pub mod get;
pub mod hello;
pub mod list;
pub mod register;
pub mod update;
