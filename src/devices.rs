crate::declare!(
	parent = crate,
	rpc = [delete],
	bidirectional_stream = [register]
);

pub mod register {
	use crate::{auth::AccessToken, SpacedriveCipherSuite};

	use std::fmt;

	use opaque_ke::{RegistrationRequest, RegistrationResponse, RegistrationUpload};
	use serde::{Deserialize, Serialize};
	use uuid::Uuid;

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
}

pub mod delete {
	use crate::auth::AccessToken;

	use serde::{Deserialize, Serialize};

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Request {
		pub access_token: AccessToken,
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Response;
}
