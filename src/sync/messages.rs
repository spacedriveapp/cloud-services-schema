crate::declare!(parent = super, rpc = [push]);

pub mod push {
	use crate::auth::AccessToken;

	use serde::{Deserialize, Serialize};
	use uuid::Uuid;

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Request {
		pub access_token: AccessToken,
		pub group_id: Uuid,
		pub encrypted_sync_messages: Vec<u8>,
		pub start_timestamp: u64,
		pub end_timestamp: u64,
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Response;
}
