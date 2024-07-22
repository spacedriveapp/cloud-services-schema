use crate::auth::AccessToken;

use serde::{Deserialize, Serialize};

use super::{Group, PubId};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
	pub with_library: bool,
	pub with_devices: bool,
	pub with_used_storage: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Response(pub Group);
