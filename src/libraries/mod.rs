use crate::devices;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

crate::declare! {
	parent = crate,
	rpc = [create, get, list, update, delete],
}

crate::need_auth!(create, get, list, update, delete);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, derive_more::Display)]
#[serde(transparent)]
pub struct PubId(pub Uuid);

#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
	pub pub_id: PubId,
	pub name: String,
	pub original_device: Option<devices::Device>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod update;
