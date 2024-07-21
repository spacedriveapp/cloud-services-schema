use quic_rpc::declare_rpc;

use super::{declare_inner_client_rpc_calls, declare_requests, declare_responses, Error};

declare_inner_client_rpc_calls!(
	crate::Service,
	rpc = [create, delete],
	client_stream = [],
	server_stream = [],
	bidirectional_stream = []
);

declare_requests!(parent -> crate::Request, rpc = [create, delete]);
declare_responses!(parent -> crate::Response, create, delete);

declare_rpc!(crate::Service, create::Request, Result<create::Response, Error>);
declare_rpc!(crate::Service, delete::Request, Result<delete::Response, Error>);

pub mod create {
	use serde::{Deserialize, Serialize};

	use crate::auth::AccessToken;

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Request {
		pub access_token: AccessToken,
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Response {}
}

pub mod delete {
	use serde::{Deserialize, Serialize};

	use crate::auth::AccessToken;

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Request {
		pub access_token: AccessToken,
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Response {}
}
