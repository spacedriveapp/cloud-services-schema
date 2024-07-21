use quic_rpc::declare_rpc;

use super::{declare_inner_client_rpc_calls, declare_requests, declare_responses, Error};

declare_inner_client_rpc_calls!(
	crate::Service,
	rpc = [delete],
	client_stream = [],
	server_stream = [],
	bidirectional_stream = [register]
);

impl<'c, C, S> Client<'c, C, S>
where
	C: ::quic_rpc::ServiceConnection<S>,
	S: ::quic_rpc::Service,
{
}

declare_requests!(parent -> crate::Request, rpc = [delete], bidirectional_stream = [register]);
declare_responses!(parent -> crate::Response, register, delete);

// declare_rpc!(crate::Service, create::Request, Result<create::Response, Error>);
declare_rpc!(crate::Service, delete::Request, Result<delete::Response, Error>);

pub mod register {
	use std::fmt;

	use opaque_ke::{RegistrationRequest, RegistrationResponse, RegistrationUpload};
	use quic_rpc::message::{BidiStreaming, BidiStreamingMsg, Msg};
	use serde::{Deserialize, Serialize};
	use uuid::Uuid;

	use crate::auth::AccessToken;
	use crate::SpacedriveCipherSuite;

	#[derive(Debug, Serialize, Deserialize)]
	pub enum DeviceOS {
		Linux,
		Windows,
		MacOS,
		IOS,
		Android,
	}

	#[derive(Serialize, Deserialize)]
	pub struct Request {
		pub access_token: AccessToken,
		pub pub_id: Uuid,
		pub name: String,
		pub os: DeviceOS,
		pub storage_size: u64,
		pub opaque_register_message: Box<RegistrationRequest<SpacedriveCipherSuite>>,
	}

	impl fmt::Debug for Request {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			f.debug_struct("Request")
				.field("access_token", &self.access_token)
				.field("pub_id", &self.pub_id)
				.field("name", &self.name)
				.field("os", &self.os)
				.field("storage_size", &self.storage_size)
				.field("opaque_register_message", &"<RegistrationRequestData>")
				.finish()
		}
	}

	#[derive(Serialize, Deserialize)]
	pub struct RequestUpdate {
		pub opaque_registration_finish: Box<RegistrationUpload<SpacedriveCipherSuite>>,
	}

	impl fmt::Debug for RequestUpdate {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			f.debug_struct("RequestUpdate")
				.field("opaque_registration_finish", &"<RegistrationUploadData>")
				.finish()
		}
	}

	#[derive(Serialize, Deserialize)]
	pub enum State {
		RegistrationResponse(Box<RegistrationResponse<SpacedriveCipherSuite>>),
		End,
	}

	impl fmt::Debug for State {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			match self {
				Self::RegistrationResponse(_) => {
					write!(f, "State::RegistrationResponse(<RegistrationResponseData>)")
				}
				Self::End => write!(f, "State::End"),
			}
		}
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Response {
		pub state: State,
	}

	impl Msg<crate::Service> for Request {
		type Pattern = BidiStreaming;
	}
	impl BidiStreamingMsg<crate::Service> for Request {
		type Update = RequestUpdate;
		type Response = Result<Response, crate::Error>;
	}
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
