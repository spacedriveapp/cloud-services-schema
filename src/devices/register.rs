use crate::{
	auth::{AccessToken, DevicePublicKey, ServerSecretKey},
	SpacedriveCipherSuite,
};

use std::fmt;

use iroh_base::ticket::NodeTicket;
use opaque_ke::{RegistrationRequest, RegistrationResponse, RegistrationUpload};
use serde::{Deserialize, Serialize};

use super::{DeviceOS, PubId};

#[derive(Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
	pub name: String,
	pub os: DeviceOS,
	pub storage_size: u64,
	pub public_key: DevicePublicKey,
	pub connection_id: NodeTicket,
	pub opaque_register_message: Box<RegistrationRequest<SpacedriveCipherSuite>>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestUpdate {
	pub opaque_registration_finish: Box<RegistrationUpload<SpacedriveCipherSuite>>,
}

#[derive(Serialize, Deserialize)]
pub enum State {
	RegistrationResponse(Box<RegistrationResponse<SpacedriveCipherSuite>>),
	End(ServerSecretKey),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response(pub State);

impl fmt::Debug for Request {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Request")
			.field("access_token", &self.access_token)
			.field("pub_id", &self.pub_id)
			.field("name", &self.name)
			.field("os", &self.os)
			.field("storage_size", &self.storage_size)
			.field("public_key", &self.public_key)
			.field("connection_id", &"REDACTED")
			.field("opaque_register_message", &"<RegistrationRequestData>")
			.finish()
	}
}

impl fmt::Debug for RequestUpdate {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("RequestUpdate")
			.field("opaque_registration_finish", &"<RegistrationUploadData>")
			.finish()
	}
}

impl fmt::Debug for State {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::RegistrationResponse(_) => {
				write!(f, "State::RegistrationResponse(<RegistrationResponseData>)")
			}
			Self::End(_) => write!(f, "State::End"),
		}
	}
}
