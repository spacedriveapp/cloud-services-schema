use crate::{
	auth::{AccessToken, DevicePublicKey, ServerSecretKey},
	SpacedriveCipherSuite,
};

use std::fmt;

use iroh_base::ticket::NodeTicket;
use opaque_ke::{CredentialFinalization, CredentialRequest, CredentialResponse};
use serde::{Deserialize, Serialize};

use super::PubId;

#[derive(Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
	pub connection_id: NodeTicket,
	pub public_key: DevicePublicKey,
	pub opaque_login_message: Box<CredentialRequest<SpacedriveCipherSuite>>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestUpdate {
	pub opaque_login_finish: Box<CredentialFinalization<SpacedriveCipherSuite>>,
}

#[derive(Serialize, Deserialize)]
pub enum State {
	LoginResponse(Box<CredentialResponse<SpacedriveCipherSuite>>),
	End(ServerSecretKey),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response(pub State);

impl fmt::Debug for Request {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Request")
			.field("access_token", &self.access_token)
			.field("pub_id", &self.pub_id)
			.field("connection_id", &"REDACTED")
			.field("public_key", &self.public_key)
			.field("opaque_login_message", &"<RegistrationRequestData>")
			.finish()
	}
}

impl fmt::Debug for RequestUpdate {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("RequestUpdate")
			.field("opaque_login_finish", &"<CredentialFinalizationData>")
			.finish()
	}
}

impl fmt::Debug for State {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::LoginResponse(_) => {
				write!(f, "State::LoginResponse(<CredentialResponseData>)")
			}
			Self::End(_) => write!(f, "State::End"),
		}
	}
}
