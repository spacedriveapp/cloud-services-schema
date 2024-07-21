use quic_rpc::declare_rpc;

use crate::{declare_inner_client_rpc_calls, declare_requests, declare_responses, sync};

declare_inner_client_rpc_calls!(
	sync::Service,
	rpc = [push],
	client_stream = [],
	server_stream = [],
	bidirectional_stream = []
);

declare_requests!(parent -> sync::Request, rpc = [push]);
declare_responses!(parent -> sync::Response, push);

declare_rpc!(sync::Service, push::Request, Result<push::Response, crate::Error>);

pub mod push {
	use serde::{Deserialize, Serialize};
	use uuid::Uuid;

	use crate::auth::AccessToken;

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
