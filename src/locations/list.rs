use crate::auth::AccessToken;

use serde::{Deserialize, Serialize};

use super::Location;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub with_library: bool,
	pub with_device: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Response(pub Vec<Location>);
