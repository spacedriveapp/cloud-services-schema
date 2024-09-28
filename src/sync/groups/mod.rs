use crate::{devices, libraries};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::KeyHash;

crate::declare! {
	parent = super,
	rpc = [
		create,
		delete,
		get,
		leave,
		list,
		remove_device,
		reply_join_request,
		request_join,
	],
}

crate::need_auth!(
	create,
	delete,
	get,
	leave,
	list,
	remove_device,
	reply_join_request,
	request_join,
);

#[derive(
	Debug,
	Clone,
	Copy,
	Serialize,
	Deserialize,
	derive_more::Display,
	specta::Type,
	PartialEq,
	Eq,
	Hash,
	PartialOrd,
	Ord,
)]
#[serde(transparent)]
#[specta(rename = "CloudSyncGroupPubId", transparent)]
pub struct PubId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[specta(rename = "CloudSyncGroup")]
pub struct Group {
	pub pub_id: PubId,
	pub latest_key_hash: KeyHash,
	pub library: Option<libraries::Library>,
	pub devices: Option<Vec<devices::Device>>,
	pub total_sync_messages_bytes: Option<u64>,
	pub total_space_files_bytes: Option<u64>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[specta(rename = "CloudSyncGroupWithLibraryAndDevices")]
pub struct GroupWithLibraryAndDevices {
	pub pub_id: PubId,
	pub latest_key_hash: KeyHash,
	pub library: libraries::Library,
	pub devices: Vec<devices::Device>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

pub mod create;
pub mod delete;
pub mod get;
pub mod leave;
pub mod list;
pub mod remove_device;
pub mod reply_join_request;
pub mod request_join;
