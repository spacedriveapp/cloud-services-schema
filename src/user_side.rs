use serde::{Deserialize, Serialize};

crate::declare!(rpc = [authorize_new_device], custom_error = UserSideError);

#[derive(Debug, Serialize, Deserialize)]
pub enum UserSideError {
	RequestRejected,
}

pub mod authorize_new_device {
	use serde::{Deserialize, Serialize};
	use uuid::Uuid;

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Request {
		new_sd_instance_id: Uuid,
		sync_group_id: Uuid,
		pgp_public_keys_pool: Vec<Vec<u8>>,
		root_aes_key: Vec<u8>,
		encrypted_sync_aes_key: Vec<u8>,
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Response {
		re_encrypted_sync_aes_key: Vec<u8>,
	}
}
