use crate::{auth::AccessToken, SpacedriveCipherSuite};

use std::fmt;

use iroh_base::key::NodeId;
use opaque_ke::{RegistrationRequest, RegistrationResponse, RegistrationUpload};
use serde::{Deserialize, Serialize};

use super::{DeviceOS, HardwareModel, PubId};

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
	pub name: String,
	pub os: DeviceOS,
	pub hardware_model: HardwareModel,
	pub connection_id: NodeId,
	pub opaque_register_message: Box<RegistrationRequest<SpacedriveCipherSuite>>,
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestUpdate {
	pub opaque_registration_finish: Box<RegistrationUpload<SpacedriveCipherSuite>>,
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum State {
	RegistrationResponse(Box<RegistrationResponse<SpacedriveCipherSuite>>),
	End,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response(pub State);

impl fmt::Debug for Request {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Request")
			.field("access_token", &self.access_token)
			.field("pub_id", &self.pub_id)
			.field("name", &self.name)
			.field("os", &self.os)
			.field("hardware_model", &self.hardware_model)
			.field("connection_id", &self.connection_id)
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
			Self::End => write!(f, "State::End"),
		}
	}
}
