crate::declare! {
	parent = super,
	rpc = [create, delete]
}

crate::need_auth!(create, delete);

pub mod create {
	use crate::auth::AccessToken;

	use serde::{Deserialize, Serialize};

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Request {
		pub access_token: AccessToken,
	}

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Response;
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
