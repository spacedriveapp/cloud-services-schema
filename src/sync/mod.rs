use quic_rpc::declare_rpc;

use super::{
	declare_client, declare_client_rpc_calls, declare_requests, declare_responses, error::Error,
};

pub mod groups;
pub mod messages;

/// RPC service declaration for [`sync::Service`]
#[derive(Clone, Debug)]
pub struct Service;

impl quic_rpc::Service for Service {
	type Req = Request;
	type Res = Response;
}

declare_rpc!(Service, get_sync_key::Request, Result<get_sync_key::Response, Error>);
declare_rpc!(Service, instance_hello::Request, Result<instance_hello::Response, Error>);

declare_client!(Service, child_clients = [messages, groups]);

declare_client_rpc_calls!(
	Client<C, S>,
	rpc = [get_sync_key, instance_hello],
	client_stream = [],
	server_stream = [],
	bidirectional_stream = []
);

declare_requests!(groups, messages, get_sync_key, instance_hello);
declare_responses!(
	children_responses = [messages, groups],
	get_sync_key,
	instance_hello,
);

pub mod get_sync_key {
	use serde::{Deserialize, Serialize};
	use uuid::Uuid;

	use crate::auth::AccessToken;

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
	use serde::{Deserialize, Serialize};
	use uuid::Uuid;

	use crate::auth::AccessToken;

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Request {
		pub access_token: AccessToken,
		pub sd_instance_id: Uuid,
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Response;
}
