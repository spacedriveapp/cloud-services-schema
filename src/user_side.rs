use quic_rpc::declare_rpc;
use serde::{Deserialize, Serialize};

use crate::{declare_client, declare_client_rpc_calls, declare_requests, declare_responses};

/// RPC service declaration for [`user_side::Service`]
#[derive(Clone, Debug)]
pub struct Service;

impl quic_rpc::Service for Service {
	type Req = Request;
	type Res = Response;
}

declare_rpc!(Service, authorize_new_instance::Request, Result<authorize_new_instance::Response, UserSideError>);

declare_client!(Service);

declare_client_rpc_calls!(
	Client<C, S>,
	UserSideError,
	rpc = [authorize_new_instance],
	client_stream = [],
	server_stream = [],
	bidirectional_stream = []
);

#[derive(Debug, Serialize, Deserialize)]
pub enum UserSideError {
	RequestRejected,
	FailedToDecryptPGPPrivateKey,
	FailedToDecryptSyncAESKey,
	FailedToEncryptSyncAESKey,
}

declare_requests!(rpc = [authorize_new_instance]);
declare_responses!(UserSideError, authorize_new_instance);

pub mod authorize_new_instance {
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
