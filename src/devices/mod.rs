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

#[derive(
	Debug, Clone, Copy, Serialize, Deserialize, derive_more::Display, specta::Type, Eq, PartialEq,
)]
#[serde(transparent)]
#[specta(rename = "DevicePubId", transparent)]
pub struct PubId(pub Uuid);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, specta::Type)]
pub enum DeviceOS {
	Linux,
	Windows,
	MacOS,
	#[serde(rename = "iOS")]
	IOS,
	Android,
}

impl DeviceOS {
	#[must_use]
	pub fn from_env() -> Self {
		match std::env::consts::OS {
			"linux" => Self::Linux,
			"macos" => Self::MacOS,
			"windows" => Self::Windows,
			"android" => Self::Android,
			"ios" => Self::IOS,
			_ => {
				// The remaining options according to docs are:
				// - freebsd
				// - dragonfly
				// - netbsd
				// - openbsd
				// - solaris
				// They aren't even supported to begin with, so we just default to Linux as
				// a close enough fallback
				Self::Linux
			}
		}
	}
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, specta::Type)]
pub enum HardwareModel {
	Other,
	MacStudio,
	MacBookAir,
	MacBookPro,
	MacBook,
	MacMini,
	MacPro,
	IMac,
	IMacPro,
	IPad,
	IPhone,
	Simulator,
	Android,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Device {
	pub pub_id: PubId,
	pub name: String,
	pub os: DeviceOS,
	pub storage_size: u64,
	pub used_storage: u64,
	#[specta(type = String)]
	pub connection_id: NodeId,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	pub hardware_model: HardwareModel,
}

pub mod delete;
pub mod get;
pub mod hello;
pub mod list;
pub mod register;
pub mod update;
