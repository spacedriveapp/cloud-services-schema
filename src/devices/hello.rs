use crate::{auth::AccessToken, SpacedriveCipherSuite};

use std::fmt;

use opaque_ke::{CredentialFinalization, CredentialRequest, CredentialResponse};
use serde::{Deserialize, Serialize};

use super::PubId;

#[derive(Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
	pub opaque_login_message: Box<CredentialRequest<SpacedriveCipherSuite>>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestUpdate {
	pub opaque_login_finish: Box<CredentialFinalization<SpacedriveCipherSuite>>,
}

#[derive(Serialize, Deserialize)]
pub enum State {
	LoginResponse(Box<CredentialResponse<SpacedriveCipherSuite>>),
	End,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response(pub State);

impl fmt::Debug for Request {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Request")
			.field("access_token", &self.access_token)
			.field("pub_id", &self.pub_id)
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
			Self::End => write!(f, "State::End"),
		}
	}
}
