pub mod groups;
pub mod messages;

crate::declare!(
	children_clients = [messages, groups],
	rpc = [get_sync_key, instance_hello],
);

pub mod get_sync_key {
	use crate::auth::AccessToken;

	use serde::{Deserialize, Serialize};
	use uuid::Uuid;

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Request {
		pub access_token: AccessToken,
		pub group_id: Uuid,
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Response {
		pub root_aes_key: Vec<u8>,
		pub encrypted_sync_aes_key: Vec<u8>,
	}
}

pub mod instance_hello {
	use crate::auth::AccessToken;

	use serde::{Deserialize, Serialize};
	use uuid::Uuid;

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Request {
		pub access_token: AccessToken,
		pub sd_instance_id: Uuid,
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Response;
}
