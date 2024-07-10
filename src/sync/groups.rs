use quic_rpc::declare_rpc;

use crate::{declare_inner_client_rpc_calls, declare_requests, declare_responses, Error, sync};

declare_inner_client_rpc_calls!(
	sync::Service,
	rpc = [create, list, join, leave, delete],
	client_stream = [],
	server_stream = [],
	bidirectional_stream = []
);

declare_requests!(parent -> sync::Request, create, list, join, leave, delete);
declare_responses!(parent -> sync::Response, create, list, join, leave, delete);

declare_rpc!(sync::Service, create::Request, Result<create::Response, Error>);
declare_rpc!(sync::Service, list::Request, Result<list::Response, Error>);
declare_rpc!(sync::Service, join::Request, Result<join::Response, Error>);
declare_rpc!(sync::Service, leave::Request, Result<leave::Response, Error>);
declare_rpc!(sync::Service, delete::Request, Result<delete::Response, Error>);

pub mod create {
	use serde::{Deserialize, Serialize};
	use uuid::Uuid;

	use crate::auth::AccessToken;

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
