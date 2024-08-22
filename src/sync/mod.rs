use serde::{Deserialize, Serialize};

pub mod groups;
pub mod messages;
pub mod space_files;

crate::declare! {
	children_clients = [messages, groups, space_files],
	rpc = [find_key_owners],
}

crate::need_auth!(find_key_owners);

#[derive(
	Debug, Clone, Serialize, Deserialize, specta::Type, Hash, Eq, PartialEq, Ord, PartialOrd,
)]
#[serde(transparent)]
#[specta(transparent)]
pub struct KeyHash(pub String);

pub mod find_key_owners {
	use crate::{auth::AccessToken, devices};

	use serde::{Deserialize, Serialize};

	use super::{groups, KeyHash};

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Request {
		pub access_token: AccessToken,
		pub group_pub_id: groups::PubId,
		pub current_device_pub_id: devices::PubId,
		pub key_hash: KeyHash,
	}

	#[derive(Debug, Serialize, Deserialize, specta::Type)]
	pub struct Response {
		pub original_key_creator_device_pub_id: devices::Device,
		pub devices_in_group: Vec<devices::Device>,
	}
}
