crate::declare!(parent = super, rpc = [create, list, join, leave, delete]);

pub mod create {
	use crate::auth::AccessToken;

	use serde::{Deserialize, Serialize};
	use uuid::Uuid;

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Request {
		pub access_token: AccessToken,
		pub sd_instance_id: Uuid,
		pub pgp_public_key: Vec<u8>,
		pub encrypted_sync_aes_key: Vec<u8>,
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Response {
		pub group_id: Uuid,
		pub root_aes_key: Vec<u8>,
	}
}

// TODO: Implement these below
pub mod list {
	use serde::{Deserialize, Serialize};

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Request {}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Response {}
}

pub mod join {
	use serde::{Deserialize, Serialize};

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Request {}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Response {}
}

pub mod leave {
	use serde::{Deserialize, Serialize};

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Request {}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Response {}
}

pub mod delete {
	use serde::{Deserialize, Serialize};

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Request {}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Response {}
}
