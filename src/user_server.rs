use serde::{Deserialize, Serialize};

crate::declare!(rpc = [authorize_new_device], custom_error = UserServerError);

#[derive(Debug, Serialize, Deserialize)]
pub enum UserServerError {
	Rejected,
	UnableToConnect,
}

pub mod authorize_new_device {
	use crate::devices::DeviceOS;

	use iroh_base::ticket::NodeTicket;
	use serde::{Deserialize, Serialize};
	use uuid::Uuid;

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Request {
		pub_id: Uuid,
		name: String,
		os: DeviceOS,
		connection_id: NodeTicket,
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Response;
}
