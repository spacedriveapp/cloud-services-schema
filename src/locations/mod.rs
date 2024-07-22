use crate::{devices, libraries};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

crate::declare!(parent = crate, rpc = [create, get, list, update, delete]);

#[derive(Debug, Serialize, Deserialize)]
pub struct PubId(pub Uuid);

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
	pub pub_id: PubId,
	pub name: String,
	pub device: Option<devices::Device>,
	pub library: Option<libraries::Library>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod update;
